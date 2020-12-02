use ssvm_tensorflow_interface;
use std::env;
use std::fs::File;
use std::io::Read;

fn main() {
    let args: Vec<String> = env::args().collect();
    let model_name: &str = &args[1];
    let image_name: &str = &args[2];

    let mut file_mod = File::open(model_name).unwrap();
    let mut mod_buf = Vec::new();
    file_mod.read_to_end(&mut mod_buf).unwrap();
    let mut file_img = File::open(image_name).unwrap();
    let mut img_buf = Vec::new();
    file_img.read_to_end(&mut img_buf).unwrap();

    let flat_img = ssvm_tensorflow_interface::load_jpg_image_to_rgb32f(&img_buf, 224, 224);
    /* The `flat_img` float vector can prepared as following:
    let img = image::load_from_memory(&img_buf).unwrap().to_rgb();
    let resized = image::imageops::resize(&img, 224, 224, ::image::imageops::FilterType::Triangle);
    let mut flat_img: Vec<f32> = Vec::new();
    for rgb in resized.pixels() {
        flat_img.push(rgb[0] as f32 / 255.);
        flat_img.push(rgb[1] as f32 / 255.);
        flat_img.push(rgb[2] as f32 / 255.);
    }

    Note that the dependencies are needed in Cargo.toml:
    image = { version = "0.23.0", default-features = false, features = ["jpeg", "png", "gif"] }
    imageproc = "0.21.0"
    */

    let mut session = ssvm_tensorflow_interface::Session::new(&mod_buf, ssvm_tensorflow_interface::ModelType::TensorFlow);
    session.add_input("input", &flat_img, &[1, 224, 224, 3])
           .add_output("MobilenetV2/Predictions/Softmax")
           .run();

    let res_vec: Vec<f32> = session.get_output("MobilenetV2/Predictions/Softmax");
    let mut i = 0;
    let mut max_index: i32 = -1;
    let mut max_value: f32 = -1.0;
    while i < res_vec.len() {
        let cur = res_vec[i];
        if cur > max_value {
            max_value = cur;
            max_index = i as i32;
        }
        i += 1;
    }
    println!("{} : {}", max_index, max_value as f64);
}
