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

    let flat_img = ssvm_tensorflow_interface::load_jpg_image_to_rgb8(&img_buf, 224, 224);

    let mut session = ssvm_tensorflow_interface::Session::new(&mod_buf, ssvm_tensorflow_interface::ModelType::TensorFlowLite);
    session.add_input("module/hub_input/images_uint8", &flat_img, &[1, 224, 224, 3])
           //.add_output("module/prediction")   For tensorflow-lite case, add_output is not needed.
           .run();

    let res_vec: Vec<u8> = session.get_output("module/prediction");
    let mut i = 0;
    let mut max_index: i32 = -1;
    let mut max_value: u8 = 0;
    while i < res_vec.len() {
        let cur = res_vec[i];
        if cur > max_value {
            max_value = cur;
            max_index = i as i32;
        }
        i += 1;
    }
    println!("{} : {}", max_index, max_value as f32 / 255.0);
}
