# The poster watermark example

In this example, we demonstrate how to create and run a Rust function in the Second State Rust Functions.

## Prerequisites

If you have not done so already, follow these simple instructions to install [Rust](https://www.rust-lang.org/tools/install) and [rustwasmc](https://www.secondstate.io/articles/ssvmup/).

## Build the WASM bytecode

```
rustwasmc build
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
{"wasm_id":349,"wasm_sha256":"0x778fae35baf1d461b1e931d3c41964908947a2b3fbde02166b50d573f1e15959","SSVM_Usage_Key":"00000000-0000-0000-0000-000000000000","SSVM_Admin_Key":"50378009-bbbe-4043-8d7e-af80d0af531a"}
```

Note: You can update this binary with the `SSVM_Admin_Key`.

```
$ curl --location --request PUT 'https://rpc.ssvm.secondstate.io:8081/api/update_wasm_binary/349' \
--header 'Content-Type: application/octet-stream' \
--header 'SSVM_Admin_Key: 7dxxxxxx-xxxx-xxxx-xxxx-xxxxxxxx0c41' \
--data-binary '@pkg/watermark_lib_bg.wasm'
```

## Insert your name

Add watermark to a local PNG image.

```
$ curl --location --request POST 'https://rpc.ssvm.secondstate.io:8081/api/multipart/run/148/watermark/bytes' \
--header 'Content-Type: multipart/form-data' \
--form 'input_1=Second State Functions' \
--output tmp.png
```


## Serverless web app

Open web page [html/index.html](html/index.html) in any browser. See a [static demo](https://sls-website-ap-hongkong-ge3c73q-1302315972.cos-website.ap-hongkong.myqcloud.com/index-en.html).

## Local Test

When you changed the parameters in `lib.rs`, you may need to test your poster. For better local tests, we added `main.rs`.

If you want to change the input text, you need to open the `main.rs`.

```
vi src/main.rs
```

Run the following commland to see the tested result.

```
cargo run
```

If everything goes well, follow the aboving instructions to publish your web application.
