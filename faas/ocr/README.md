# The email example

In this example, we will demonstrate how to detect characters from an image i.e. Optical Character Recognition (OCR).

## Prerequisites

If you have not done so already, follow these simple instructions to install [Rust](https://www.rust-lang.org/tools/install) and [ssvmup](https://www.secondstate.io/articles/ssvmup/).

## Build the WASM bytecode
Notice that we are using the `--enable-ext` flag which will use `ssvm-extensions` instead of `ssvm`.

```
$ ssvmup build --enable-ext
```

## Create FaaS function

Upload the wasm file in the `pkg` folder to the FaaS. Double check the `.wasm` file name before you upload.

```
curl --location --request POST 'https://rpc.ssvm.secondstate.io:8081/api/executables' \
--header 'Content-Type: application/octet-stream' \
--header 'SSVM-Description: OCR' \
--data-binary '@pkg/ocr_lib_bg.wasm'
```

Returns

```
{"wasm_id":281,"wasm_sha256":"0x39cfdbe0d0aa31d87e81d72506fa88af5ab6f3ba82b3d09f5330aac8ba061673","SSVM_Usage_Key":"00000000-0000-0000-0000-000000000000","SSVM_Admin_Key":"xxxxxx-de44-4fc8-abf7-03f61f648b71"}
```

Note: You can update this binary with the `SSVM_Admin_Key`.

```
curl --location --request PUT 'https://rpc.ssvm.secondstate.io:8081/api/update_wasm_binary/281' \
--header 'Content-Type: application/octet-stream' \
--header 'SSVM_Admin_Key: xxxxxx-de44-4fc8-abf7-03f61f648b71' \
--data-binary '@pkg/ocr_lib_bg.wasm'
```

## Test

Make a function call via the web.

```
curl --location --request POST 'https://rpc.ssvm.secondstate.io:8081/api/run/281/ocr' \
--header 'Content-Type: application/octet-stream' \
--data-binary '@html/ocr.jpg'
```

## Live demo

Please click on [this HTML link](https://second-state.github.io/wasm-learning/faas/ocr/html/index.html) which will take you to the live demonstration.
