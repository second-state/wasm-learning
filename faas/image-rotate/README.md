# The image rotation example

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
--header 'SSVM_Description: Image rotate' \
--data-binary '@pkg/rotate_lib_bg.wasm'
```

Returns

```
{"wasm_id":232,"wasm_sha256":"0x0a3227cd8d76c32f4788ca8d020091f89c41f4abc7a3c3b1c10490d439a22b1b","SSVM_Usage_Key":"00000000-0000-0000-0000-000000000000","SSVM_Admin_Key":"aaaa-bbbb-cccc-dddd-0000"}
```

Note: You can update this binary with the `SSVM_Admin_Key`.

```
curl --location --request PUT 'https://rpc.ssvm.secondstate.io:8081/api/update_wasm_binary/232' \
--header 'Content-Type: application/octet-stream' \
--header 'SSVM_Admin_Key: aaaa-bbbb-cccc-dddd-0000' \
--data-binary '@pkg/rotate_lib_bg.wasm'
```

## Rotate the image

Rotate the image and save result to a local PNG image.

```
curl --location --request POST 'https://rpc.ssvm.secondstate.io:8081/api/multipart/run/232/rotate_an_image/bytes' \
--header 'Content-Type: multipart/form-data' \
--form 'input_1=@html/lean.png' \
--output tmp.png
```



## Serverless web app

Open web page [html/index.html](html/index.html) in any browser. See a [static demo](https://second-state.github.io/wasm-learning/faas/image-rotate/html/index.html).
