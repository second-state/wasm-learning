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

    let flat_img = ssvm_tensorflow_interface::load_jpg_image_to_rgb8(&img_buf, 640, 640);
    let mut args = ssvm_tensorflow_interface::SessionArgs::new();
    args.add_input("input_tensor", &flat_img, &[1, 640, 640, 3]);
    args.add_output("StatefulPartitionedCall:1");
    args.add_output("StatefulPartitionedCall:2");
    args.add_output("StatefulPartitionedCall:4");

    let res = ssvm_tensorflow_interface::exec_model(&mod_buf, &args);
    let detection_boxes: Vec<f32> = res.get_output("StatefulPartitionedCall:1");
    let detection_classes: Vec<f32> = res.get_output("StatefulPartitionedCall:2");
    let detection_scores: Vec<f32> = res.get_output("StatefulPartitionedCall:4");

    println!("Boxes : {:?}", detection_boxes);
    println!("Classes : {:?}", detection_classes);
    println!("Scores : {:?}", detection_scores);
}
