# The image watermark example

In this example, we demonstrate how to create and run a Rust function in the Second State Rust FaaS.

## Prerequisites

If you have not done so already, follow these simple instructions to install [Rust](https://www.rust-lang.org/tools/install) and [ssvmup](https://www.secondstate.io/articles/ssvmup/).

## Build the WASM bytecode

```
$ ssvmup build
```

## FaaS

Upload the wasm file in the `pkg` folder to the FaaS. Double check the `.wasm` file name before you upload.

```
$ curl --location --request POST 'https://rpc.ssvm.secondstate.io:8081/api/executables' \
--header 'Content-Type: application/octet-stream' \
--header 'SSVM-Description: watermark' \
--data-binary '@pkg/watermark_lib_bg.wasm'
```

Returns

```
{"wasm_id":97,"wasm_sha256":"0xfb413547a8aba56d0349603a7989e269f3846245e51804932b3e02bc0be4b665","usage_key":"00000000-0000-0000-0000-000000000000","admin_key":"00xxxxxx-xxxx-xxxx-xxxx-4adc960fd2b8"}
```

## Setup the watermark text

Add watermark to a local PNG image.

```
$ curl --location --request POST 'https://rpc.ssvm.secondstate.io:8081/api/multipart/run/148/watermark/bytes' \
--header 'Content-Type: application/json' \
--form 'input_1=Meow Human!' \
--form 'input_2=@test/cat.png' \
--output tmp.png
```

Make a pre-fetched FaaS call to add watermark to an Internet image.

```
$ curl --location --request POST 'https://rpc.ssvm.secondstate.io:8081/api/multipart/run/148/watermark/bytes' \
--header 'Content-Type: application/json' \
--form 'input_1=Woof Human!' \
--form 'fetch_input_2=https://www.secondstate.io/demo/dog.png' \
--output tmp.png
```

## Serverless web app

Open web page [html/index.html](html/index.html) in any browser. See a [static demo](https://www.secondstate.io/demo/2020-watermark.html).
