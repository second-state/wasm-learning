# The image watermark example

In this example, we demonstrate how to create and run a Rust function in the Second State Rust FaaS.

## Prerequisites

If you have not done so already, follow these simple instructions to install [Rust](https://www.rust-lang.org/tools/install) and [ssvmup](https://www.secondstate.io/articles/ssvmup/).

## Build the WASM bytecode

```
ssvmup build
```

## FaaS

Upload the wasm file in the `pkg` folder to the FaaS. Double check the `.wasm` file name before you upload.

```
curl --location --request POST 'https://rpc.ssvm.secondstate.io:8081/api/executables' \
--header 'Content-Type: application/octet-stream' \
--header 'SSVM-Description: watermark' \
--data-binary '@pkg/watermark_lib_bg.wasm'
```

Returns

```
{"wasm_id":154,"wasm_sha256":"0x0a3227cd8d76c32f4788ca8d020091f89c41f4abc7a3c3b1c10490d439a22b1b","SSVM_Usage_Key":"00000000-0000-0000-0000-000000000000","SSVM_Admin_Key":"aaaa-bbbb-cccc-dddd-0000"}
```

## Setup the watermark text

Add watermark to a local PNG image.

```
curl --location --request POST 'https://rpc.ssvm.secondstate.io:8081/api/multipart/run/154/watermark/bytes' \
--header 'Content-Type: multipart/form-data' \
--form 'input_1=Howdy Second State' \
--form 'input_2=@test/cat.png' \
--output tmp.png
```

Make a pre-fetched FaaS call to add watermark to an Internet image.

```
curl --location --request POST 'https://rpc.ssvm.secondstate.io:8081/api/multipart/run/154/watermark/bytes' \
--header 'Content-Type: multipart/form-data' \
--form 'input_1=Howdy Second State' \
--form 'fetch_input_2=https://www.secondstate.io/demo/dog.png' \
--output tmp.png
```

## Serverless web app

Open web page [html/index.html](html/index.html) in any browser. See a [static demo](https://www.secondstate.io/demo/2020-watermark.html).
