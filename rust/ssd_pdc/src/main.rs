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

    let flat_img = ssvm_tensorflow_interface::load_jpg_image_to_rgb32f(&img_buf, 256, 256);
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
    let mut args = ssvm_tensorflow_interface::SessionArgs::new();
    args.add_input("Preprocessor/sub", &flat_img, &[1, 256, 256, 3]);
    args.add_output("concat");
    args.add_output("concat_1");

    let res = ssvm_tensorflow_interface::exec_model(&mod_buf, &args);
    let concat_vec: Vec<f32> = res.get_output("concat");
    let concat_1_vec: Vec<f32> = res.get_output("concat_1");

    println!("Output concat : {:?}", concat_vec);
    println!("Output concat_1 : {:?}", concat_1_vec);
}
