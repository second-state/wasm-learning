use rust_process_interface_library::Command;
use ssvm_wasi_helper::ssvm_wasi_helper::_initialize;
use std::fs::File;
use std::io::Read;
use std::str;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn ocr(img_buf: &[u8]) -> String {
    _initialize();
    match image::load_from_memory(img_buf) {
        Ok(img_to_write) => {
            img_to_write.save_with_format("/temp.png", image::ImageFormat::Png);
        }
        Err(e) => println!("Error: {:?}", &e.to_string()),
    };
    let mut cmd = Command::new("tesseract");
    cmd.arg("/temp_input.png").arg("/temp_output");
    let mut f = File::open("/temp.txt").unwrap();
    let mut s = String::new();
    f.read_to_string(&mut s).unwrap();
    s
}
