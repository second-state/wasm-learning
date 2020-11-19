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

    let img = image::load_from_memory(&img_buf).unwrap().to_rgb();
    let resized = image::imageops::resize(&img, 640, 640, ::image::imageops::FilterType::Triangle);
    let mut flat_img: Vec<u8> = Vec::new();
    for rgb in resized.pixels() {
        flat_img.push(rgb[0]);
        flat_img.push(rgb[1]);
        flat_img.push(rgb[2]);
    }

    let res = ssvm_tensorflow_interface::run_tensorflow_vision(
        &mod_buf,
        &flat_img,
        &[1, 640, 640, 3],
        640,
        640,
        "input_tensor",
        &["StatefulPartitionedCall:1","StatefulPartitionedCall:2","StatefulPartitionedCall:4"]
    );
    let detection_boxes: Vec<f32> = res.convert_to_vec(0);
    let detection_classes: Vec<f32> = res.convert_to_vec(1);
    let detection_scores: Vec<f32> = res.convert_to_vec(2);

    println!("Boxes : {:?}", detection_boxes);
    println!("Classes : {:?}", detection_classes);
    println!("Scores : {:?}", detection_scores);
}
