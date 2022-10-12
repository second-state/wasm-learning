# The MTCNN example

Run MTCNN tensorflow models as functions.

[Live Demo](https://second-state.github.io/wasm-learning/faas/mtcnn/html/index.html) | [Code Tutorial](https://www.secondstate.io/articles/faas-face-detection/)

## Prerequisites

If you have not done so already, follow these simple instructions to install [Rust](https://www.rust-lang.org/tools/install) and [rustwasmc](https://www.secondstate.io/articles/ssvmup/).

## Build the WASM bytecode

```
$ rustwasmc build --enable-ext
```

## Create FaaS function

Upload the wasm file in the `pkg` folder to the FaaS. Double check the `.wasm` file name before you upload.

```
$ curl --location --request POST 'https://rpc.ssvm.secondstate.io:8081/api/executables' \
--header 'Content-Type: application/octet-stream' \
--header 'SSVM-Description: mtcnn' \
--data-binary '@pkg/mtcnn_service_lib_bg.wasm'
```

Returns

```
{"wasm_id":482,"wasm_sha256":"0x469c28daae7aba392076b4bc5ee3b43ec6d667083d8ae63207bf74b1da03fc26","SSVM_Usage_Key":"00000000-0000-0000-0000-000000000000","SSVM_Admin_Key":"7dxxxxxx-xxxx-xxxx-xxxx-xxxxxxxx0c41"}
```

Note: You can update this binary with the `SSVM_Admin_Key`.

```
$ curl --location --request PUT 'https://rpc.ssvm.secondstate.io:8081/api/update_wasm_binary/195' \
--header 'Content-Type: application/octet-stream' \
--header 'SSVM_Admin_Key: 7dxxxxxx-xxxx-xxxx-xxxx-xxxxxxxx0c41' \
--data-binary '@pkg/mtcnn_service_lib_bg.wasm'
```

## Test

Make a function call via the web.

```
$ curl --location --request POST 'https://rpc.ssvm.secondstate.io:8081/api/run/482/infer/bytes' \
--header 'Content-Type: application/octet-stream' \
--data-binary '@test/solvay.jpg' \
--output tmp.jpg
```

## Local test

You must have Node.js and NPM installed. Install dependencies.

```
$ sudo apt install -y libjpeg-dev libpng-dev
$ wget https://storage.googleapis.com/tensorflow/libtensorflow/libtensorflow-cpu-linux-x86_64-2.3.0.tar.gz
$ sudo tar -C /usr/local -xzf libtensorflow-cpu-linux-x86_64-2.3.0.tar.gz
$ sudo ldconfig
```

Install wasmedge-extensions from source

You will need to alter the Git configuration on the machine where this installation procedure is being performed. Reason being, this machine will not have the SSH keys to communicate with git@github. If you add the following config, you will be able to successfully run the `npm install --build-from-source ...` below.

```bash
git config --global url."https://github.com/".insteadOf git@github.com:
git config --global url."https://".insteadOf git://
```

You can check that this config worked by typing 

```bash
git config -l
```

Temporarily downgrade npm

The following installation will require that npm is downgraded to `6.14.9`. There is [an npm cli issue](https://github.com/npm/cli/issues/1865) which prevents us from using the latest npm for this particular build from source task.

Because of the complexity of dependency management, please install aptitude because it provides a way to automatically resolve depencency conflicts.

```bash
sudo apt install aptitude
sudo aptitude install npm
```

The following command is used to alter the npm version.

```bash
sudo npm install -g npm@6.14.9
```

Once you have temporarily downgraded npm, please go ahead and install the latest wasmedge-extensions like this

```bash
git clone --recurse-submodules https://github.com/second-state/wasmedge-extensions.git
npm install --build-from-source wasmedge-extensions
```

Run the local test on Node.js.

```bash
$ cd test
$ node test.js
Drawing box: 30 results ...
Face Detection: 888.961ms
```

---

# Appendix A

This section shows how to use wasmedge_bindgen as apposed to wasm_bingen.

## Lib.rs

Updates to the lib.rs file are as follows

```
use wasmedge_bindgen::*;
use wasmedge_bindgen_macro::*;
use wasmedge_tensorflow_interface;
use image::{GenericImageView, Pixel};
use imageproc::drawing::draw_hollow_rect_mut;
use imageproc::rect::Rect;
use std::str;
use std::time::{Instant};

#[wasmedge_bindgen]
pub fn infer(image_data: Vec<u8>) -> Result<Vec<u8>, String> {
    let start = Instant::now();
    let mut img = image::load_from_memory(&image_data[..]).unwrap();
    let mut flat_img: Vec<f32> = Vec::new();
    for (_x, _y, rgb) in img.pixels() {
        flat_img.push(rgb[2] as f32);
        flat_img.push(rgb[1] as f32);
        flat_img.push(rgb[0] as f32);
    }
    println!("Loaded image in ... {:?}", start.elapsed());

    let model_data: &[u8] = include_bytes!("mtcnn.pb");

    let mut session = wasmedge_tensorflow_interface::Session::new(model_data, wasmedge_tensorflow_interface::ModelType::TensorFlow);
    session.add_input("min_size", &[20.0f32], &[])
           .add_input("thresholds", &[0.6f32, 0.7f32, 0.7f32], &[3])
           .add_input("factor", &[0.709f32], &[])
           .add_input("input", &flat_img, &[img.height().into(), img.width().into(), 3])
           .add_output("box")
           .add_output("prob")
           .run();
    let res_vec: Vec<f32> = session.get_output("box");

    // Parse results.
    let mut iter = 0;
    let mut box_vec: Vec<[f32; 4]> = Vec::new();
    while (iter * 4) < res_vec.len() {
        box_vec.push([
            res_vec[4 * iter + 1], // x1
            res_vec[4 * iter],     // y1
            res_vec[4 * iter + 3], // x2
            res_vec[4 * iter + 2], // y2
        ]);
        iter += 1;
    }
    println!("Parsed results in ... {:?}", start.elapsed());

    println!("Drawing box: {} results ...", box_vec.len());
    let line = Pixel::from_slice(&[0, 255, 0, 0]);
    for i in 0..box_vec.len() {
        let xy = box_vec[i];
        let x1: i32 = xy[0] as i32;
        let y1: i32 = xy[1] as i32;
        let x2: i32 = xy[2] as i32;
        let y2: i32 = xy[3] as i32;
        let rect = Rect::at(x1, y1).of_size((x2 - x1) as u32, (y2 - y1) as u32);
        draw_hollow_rect_mut(&mut img, rect, *line);
    }
    
    let mut buf = Vec::new();
    img.write_to(&mut buf, image::ImageOutputFormat::Jpeg(80u8)).expect("Unable to write");
    println!("Drawn on image in ... {:?}", start.elapsed());

    return Ok(buf);
}
```

You can see that we import the wasmedge components

```
use wasmedge_bindgen::*;
use wasmedge_bindgen_macro::*;
```

Then we change the first two lines where the function is defined

```
#[wasmedge_bindgen]
pub fn infer(image_data: Vec<u8>) -> Result<Vec<u8>, String> {
```

We also update how the image is read (due to the function parameters being different)

```
let mut img = image::load_from_memory(&image_data[..]).unwrap();
```

Finally we return a `Result` (which complies with the function definition above) instead of just returning raw data

```
return Ok(buf);
```

## Cargo.toml

The new wasmedge components have to be added to Cargo.toml file also

```
wasmedge-bindgen = "0.1.8"
wasmedge-bindgen-macro = "0.1.8"
```

## Rust/Wasm system and environment changes

We no longer need to use `rustwasmc` to compile, we can use `cargo` if the following system configuration is met

First install latest version of Rust

```
curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh
```

Add the wasm32-wasi target

```
rustup target add wasm32-wasi
```

Compile using cargo

```
cargo build --target wasm32-wasi --release
```

## Uploading to FaaS

The following slightly modified HTTP request can be used to run the Wasm on FaaS

```
curl --location --request POST 'https://rpc.ssvm.secondstate.io:8081/api/executables' \
--header 'Content-Type: application/octet-stream' \
--header 'SSVM-Description: MTCNN' \
--data-binary '@target/wasm32-wasi/release/mtcnn_service_lib.wasm'
```
