# RUST,YOLO3,PYTORCH

## Concepts

```bash
1.PyTorch has LibTorch which is a C++ API.
2.C/C++ -> Rust FFI to generate bindings.
3.tch-rs which provides wrapper functions for idiomatic Rust.
```

## Setup

### Library Versions	

```bash
LibTorch	v1.8.0
tch-rs	v0.4.0
torch-sys	v0.4.0
```

first, Download LibTorch from Source
https://pytorch.org/get-started/locally/ with litorch c++ version(only cpu).
then unzip the zip file.
second, Set environment variables
Linux:
```bash
export LIBTORCH=/path/to/libtorch
export LD_LIBRARY_PATH=${LIBTORCH}/lib:$LD_LIBRARY_PATH
```
third, Set your Cargo.toml
```bash
[dependencies]
libc = "0.2.0"
tch = { version = "0.4.0", path = "your_code_directory/tch-rs" }
anyhow = "1.0"
[dev-dependencies]
torch-sys = {version="0.4.0", path="your_code_directory/torch-sys"}
```
## Model Inference
In src/main.rs file, line 128    
```bash
let original_image = image::load("ride.png")?;
```
is your own image path, so if you want to change this, just put your image into the current directory,and change the path of load("").
then exec the following code
```bash
cargo run 
```
