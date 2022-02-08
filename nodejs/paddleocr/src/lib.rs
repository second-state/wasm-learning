use ssvm_process_interface::Command;
//use ssvm_wasi_helper::ssvm_wasi_helper::_initialize;
//use std::fs;
//use std::fs::File;
//use std::io::prelude::*;
use std::str;
//use std::time::{SystemTime, UNIX_EPOCH};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn text(data: &str) -> String {
/*
    _initialize();
    let mut guest_temp_input_filename = String::from("/");
    let filename:&str="/demo10.jpg";

    guest_temp_input_filename.push_str(filename);
    let copy_of_guest_temp_input_filename = guest_temp_input_filename.clone();

    let mut pos = 0;
    let mut buffer = File::create(filename).unwrap();
    while pos < data.len() {

    
        let bytes_written = buffer.write(&data[pos..]).unwrap();
        pos += bytes_written;
    }
    //buffer.write_all(data).unwrap();
    //let f= File::open(filename).unwrap();
    //println!("{:?}",f);
    let mut host_temp_input_filename = String::from("/tmp/");
    host_temp_input_filename.push_str(filename);
    let copy_of_host_temp_input_filename = host_temp_input_filename.clone();
//  
//  println!("{:?}",copy_of_host_temp_input_filename);
*/    
    let mut cmd = Command::new("paddleocr");
    cmd.arg("--image_dir")
        .arg(data);
    let out = cmd.output();

    //let mut items=Vec::<&str>::new();
    let mut itemsb=Vec::<&str>::new();
    let mut itemsc=Vec::<&str>::new();
    let init_result:&str=str::from_utf8(&out.stdout).unwrap();
    //println!("{:?}",b);
    let init_result_iter:Vec<&str>=init_result.split("\n").collect();
    for (i,item) in init_result_iter.iter().enumerate(){
        if i>=5{
            itemsb.push(item);
            //println!("{:?}",item);
            
        }
    }
    
    for item in itemsb.iter(){
        let itemsb_slice:Vec<&str>=item.split(" ").collect();
        for (i,item) in itemsb_slice.iter().enumerate(){
            item.to_string().pop();
            if i>=12{itemsc.push(item);}
        }
    }
    //for item in itemsc.iter(){println!("{:?}",item);}
    let str_sd:String=itemsc.join(" ");
    //for item in itemsd.iter(){println!("{:?}",item);}
    let c_iter:Vec<&str>=str_sd.split(")] (\'").collect();
    //let str_sc:String=String::new();
    let mut itemsd=Vec::<&str>::new();
    for item in c_iter.iter(){
        let mut itemsf:Vec<&str>=item.split("\',").collect();
        itemsf.pop();
        println!("{:?}",itemsf);
        itemsd=itemsf;
    }
    
    //println!("{:?}",str::from_utf8(&(itemsd.join(",").as_bytes())).unwrap().to_string());
    str::from_utf8(&(itemsd.join(",").as_bytes())).unwrap().to_string()
    //itemsd

}
#[wasm_bindgen]
pub fn bounding_box(data:&str)->String{
/*    
    let mut guest_temp_input_filename = String::from("/");
    let filename:&str="demo.jpg";
    guest_temp_input_filename.push_str(filename);
    let copy_of_guest_temp_input_filename = guest_temp_input_filename.clone();
    let mut pos = 0;
    let mut buffer = File::create(guest_temp_input_filename).unwrap();
    while pos < data.len() {
        let bytes_written = buffer.write(&data[pos..]).unwrap();
        pos += bytes_written;
    }
    let mut host_temp_input_filename = String::from("/tmp/");
    host_temp_input_filename.push_str(filename);
    let copy_of_host_temp_input_filename = host_temp_input_filename.clone();
*/
    let mut cmd = Command::new("paddleocr");
    cmd.arg("--image_dir")
        .arg(data);
    let out = cmd.output();
    let init_result:&str=str::from_utf8(&out.stdout).unwrap();
    let init_result_iter:Vec<&str>=init_result.split("\n").collect();
    let mut itemsa=Vec::<&str>::new();
    for (i,item) in init_result_iter.iter().enumerate(){
        if i>=5{
            itemsa.push(item);
        }
    }
    let mut itemsb=Vec::<&str>::new();
    for item in itemsa.iter(){
        let itemsb_slice:Vec<&str>=item.split(" ").collect();
        for (i,item) in itemsb_slice.iter().enumerate(){
            item.to_string().pop();
            if i>=4{
                itemsb.push(item);
            }
        }
    }
    let str_sd:String=itemsb.join(" ");
    let c_iter:Vec<&str>=str_sd.split("] ").collect();
    for line in c_iter.iter(){
        println!("{:?}",line);
    }
    str::from_utf8(&out.stdout).unwrap().to_string()
}
