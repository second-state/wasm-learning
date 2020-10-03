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
--data-binary '@pkg/hello_watermark_lib_bg.wasm'
```

Returns

```
{"wasm_id":149,"wasm_sha256":"0xfb413547a8aba56d0349603a7989e269f3846245e51804932b3e02bc0be4b665","usage_key":"00000000-0000-0000-0000-000000000000","admin_key":"00xxxxxx-xxxx-xxxx-xxxx-4adc960fd2b8"}
```

Note: You can update this binary with the `SSVM_Admin_Key`.

```
$ curl --location --request PUT 'https://rpc.ssvm.secondstate.io:8081/api/update_wasm_binary/149' \
--header 'Content-Type: application/octet-stream' \
--header 'SSVM_Admin_Key: 7dxxxxxx-xxxx-xxxx-xxxx-xxxxxxxx0c41' \
--data-binary '@pkg/hello_watermark_lib_bg.wasm'
```

## Call the function

Add watermark to a local PNG image.

```
$ curl --location --request POST 'https://rpc.ssvm.secondstate.io:8081/api/run/149/watermark/bytes' \
--header 'Content-Type: application/octet-stream' \
--data-binary '@test/cat.png' --output tmp.png
```

**Please note** - we are calling the `/bytes` endpoint here, because the `decode` function of Rust source code returns bytes as apposed to the encode function which returned a string.

Make a pre-fetched FaaS call to add watermark to an Internet image.

```
$ curl --location --request POST 'https://rpc.ssvm.secondstate.io:8081/api/run/149/watermark/bytes' \
--header 'SSVM_Fetch: https://www.secondstate.io/demo/dog.png' --output tmp.png
```

## Serverless web app

Open web page [html/index.html](html/index.html) in any browser. See a [static demo](https://second-state.github.io/wasm-learning/faas/hello-watermark/html/index.html).
