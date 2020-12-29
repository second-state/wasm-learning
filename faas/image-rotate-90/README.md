# The rotate image example

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
--header 'SSVM-Description: brighten' \
--data-binary '@pkg/rotate_90_lib_bg.wasm'
```

Returns

```
{"wasm_id":269,"wasm_sha256":"0x0911713ed4754886166afc52e1aff20a4bc53b1239f18ed6538207fc03da0dee","SSVM_Usage_Key":"00000000-0000-0000-0000-000000000000","SSVM_Admin_Key":"x9576x32-2aab-48c6-a537-e6f1aea4fx1d"}
```
Note: You can update this binary with the `SSVM_Admin_Key`.

```
$ curl --location --request PUT 'https://rpc.ssvm.secondstate.io:8081/api/update_wasm_binary/269' \
--header 'Content-Type: application/octet-stream' \
--header 'SSVM_Admin_Key: x9576x32-2aab-48c6-a537-e6f1aea4fx1d' \
--data-binary '@pkg/rotate_90_lib_bg.wasm'
```

## Call the Function


```
curl --location --request POST 'https://rpc.ssvm.secondstate.io:8081/api/multipart/run/269/rotate_90/bytes' \
--header 'Content-Type: multipart/form-data' \
--form 'input_1=@html/surf.png' \
--output tmp.png
```

## Serverless web app

Open web page [html/index.html](html/index.html) in any browser. See a [static demo](https://second-state.github.io/wasm-learning/faas/image-rotate-90/html/index.html).
