# The flipping image example

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
--header 'SSVM-Description: flip' \
--data-binary '@pkg/flip_lib_bg.wasm'
```

Returns

```
{"wasm_id":272,"wasm_sha256":"0x84d8eeb1a9ca3fc4898c99a2e61b4a2329a83a3d4f907ae5e96155457e595342","SSVM_Usage_Key":"00000000-0000-0000-0000-000000000000","SSVM_Admin_Key":"xxxxxxxx-55b7-4227-a847-255fd26a0cfc"}```
```
Note: You can update this binary with the `SSVM_Admin_Key`.

```
curl --location --request PUT 'https://rpc.ssvm.secondstate.io:8081/api/update_wasm_binary/272' \
--header 'Content-Type: application/octet-stream' \
--header 'SSVM_Admin_Key: xxxxxxxx-55b7-4227-a847-255fd26a0cfc' \
--data-binary '@pkg/flip_lib_bg.wasm'
```

## Call the Function


```
curl --location --request POST 'https://rpc.ssvm.secondstate.io:8081/api/multipart/run/272/flip_h/bytes' \
--header 'Content-Type: multipart/form-data' \
--form 'input_1=@html/oxide.png' \
--output tmp.png
```

## Serverless web app

Open web page [html/index.html](html/index.html) in any browser. See a [static demo](https://second-state.github.io/wasm-learning/faas/image-flip/html/index.html).
