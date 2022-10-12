# The MobileNet example

Run MobileNet tensorflow models as functions.

## Prerequisites

If you have not done so already, follow these simple instructions to install [Rust](https://www.rust-lang.org/tools/install) and [rustwasmc](https://www.secondstate.io/articles/ssvmup/).

## Build the WASM bytecode

```
rustwasmc build --enable-ext
```

## Create FaaS function

Upload the wasm file in the `pkg` folder to the FaaS. Double check the `.wasm` file name before you upload.

```
curl --location --request POST 'https://rpc.ssvm.secondstate.io:8081/api/executables' \
--header 'Content-Type: application/octet-stream' \
--header 'SSVM-Description: mobilenet' \
--data-binary '@pkg/mobilenet_service_lib_bg.wasm'
```

Returns

```
Output:
{"wasm_id":205,"wasm_sha256":"0x469c28daae7aba392076b4bc5ee3b43ec6d667083d8ae63207bf74b1da03fc26","SSVM_Usage_Key":"00000000-0000-0000-0000-000000000000","SSVM_Admin_Key":"7dxxxxxx-xxxx-xxxx-xxxx-xxxxxxxx0c41"}
```

Note: You can update this binary with the `SSVM_Admin_Key`.

```
curl --location --request PUT 'https://rpc.ssvm.secondstate.io:8081/api/update_wasm_binary/205' \
--header 'Content-Type: application/octet-stream' \
--header 'SSVM_Admin_Key: 3axxxxxx-xxxx-xxxx-xxxx-xxxxxxxx463e' \
--data-binary '@pkg/mobilenet_service_lib_bg.wasm'
```

## Test

Make a function call via the web.

```
$ curl --location --request POST 'https://rpc.ssvm.secondstate.io:8081/api/run/205/infer' \
--header 'Content-Type: application/octet-stream' \
--data-binary '@test/PurpleGallinule.jpg'
```

## Local test

You must have Node.js and NPM installed. Install SSVM extensions and dependencies.

```
$ sudo apt install -y libjpeg-dev libpng-dev
$ wget https://storage.googleapis.com/tensorflow/libtensorflow/libtensorflow-cpu-linux-x86_64-2.3.0.tar.gz
$ sudo tar -C /usr/local -xzf libtensorflow-cpu-linux-x86_64-2.3.0.tar.gz
$ sudo ldconfig
$ npm i ssvm-extensions
```

Run the local test on Node.js.

```
$ cd test
$ node test.js
576 : 0.8821619
Result is:  It is very likely a <a href='https://www.google.com/search?q=Porphyrio martinicus'>Porphyrio martinicus</a> in the picture
Image Classification: 431.710ms
```

