use std::env;
use std::fs;
use std::fs::File;
use std::io::{Write, Read};

fn main() {
    println!("This is a demo application to show how to run a standalone wasi program with ssvm-napi!");
    println!("============================================");
    println!("Print environment variables");
    println!("--------------------------------------------");
    println!("The env vars are as follows.");
    for (key, value) in env::vars() {
        println!("{}: {}", key, value);
    }
    println!("============================================\n");
    println!("Print arguments");
    println!("--------------------------------------------");
    println!("The args are as follows.");
    for argument in env::args() {
        println!("{}", argument);
    }
    println!("============================================\n");
    println!("Test filesystem, create a /hello.txt, read and write to it, and then delete it");
    println!("--------------------------------------------");
    let path = "/hello.txt".to_string();
    let content = "Hello from SSVM\nThis file is located at wasm binary folder".to_string();
    let mut output = File::create(&path).unwrap();
    output.write_all(&content.as_bytes()).unwrap();
    let mut f = File::open(&path).unwrap();
    let mut s = String::new();
    let ret = match f.read_to_string(&mut s) {
        Ok(_) => s,
        Err(e) => e.to_string(),
    };
    println!("Output: {}", ret);
    fs::remove_file(&path).expect("Unable to delete");
    println!("============================================\n");
}
