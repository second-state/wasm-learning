use std::io::{self, Read};
use ssvm_tensorflow_interface;
use serde::Deserialize;

fn main() {
    let model_data: &[u8] = include_bytes!("logo_tiktok.tflite");

    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer).expect("Error reading from STDIN");
    let obj: FaasInput = serde_json::from_str(&buffer).unwrap();
    // println!("{} {}", &(obj.body)[..5], obj.body.len());
    let img_buf = base64::decode_config(&(obj.body), base64::STANDARD).unwrap();
    // println!("Image buf size is {}", img_buf.len());

    let flat_img_u8 = ssvm_tensorflow_interface::load_jpg_image_to_rgb8(&img_buf, 180, 180);
    let mut flat_img: Vec<f32> = Vec::new();
    for p in flat_img_u8 {
        flat_img.push(p as f32);
    }
    // println!("{:?}", flat_img);

    let mut session = ssvm_tensorflow_interface::Session::new(&model_data, ssvm_tensorflow_interface::ModelType::TensorFlowLite);
    session.add_input("input_2", &flat_img, &[1, 180, 180, 3])
           .run();
    let res_vec: Vec<f32> = session.get_output("Identity");

    // println!("{:?}", res_vec);
    let detected = 1.0 - res_vec[0];
    let mut confidence = "几乎肯定没有";
    if detected > 0.75 {
        confidence = "几乎肯定有";
    } else if detected > 0.5 {
        confidence = "可能有";
    } else if detected > 0.25 {
        confidence = "可能没有";
    }
    println!("图片中{} Tiktok 的商标与水印。", confidence.to_string());

    /*
    let mut confidence = "almost certainly does NOT";
    if detected > 0.75 {
        confidence = "almost certainly" {
    } else if detected > 0.5 {
        confidence = "could";
    } else if detected > 0.25 {
        confidence = "probably does not";
    }
    println!("The uploaded photo {} have a Tiktok logo and watermark.", confidence.to_string());
    */
}

#[derive(Deserialize, Debug)]
struct FaasInput {
    body: String
}
