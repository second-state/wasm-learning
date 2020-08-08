# The image recognition example

In this example, we demonstrate how to create and run a Rust function in the Second State Rust FaaS.

## Prerequisites

If you have not done so already, follow these simple instructions to install [Rust](https://www.rust-lang.org/tools/install) and [ssvmup](https://www.secondstate.io/articles/ssvmup/).

## Build the WASM bytecode

```
$ ssvmup build
```

## Test

Run a local test as follows. You must have the [Node.js and SSVM installed](https://www.secondstate.io/articles/setup-rust-nodejs/) for this to work.

```
$ cd test
$ node test.js
Detected object military uniform with probability 0.3256046
```

## FaaS

Upload the wasm file to the FaaS.

```
$ curl --location --request POST 'https://rpc.ssvm.secondstate.io:8081/api/executables' \
--header 'Content-Type: application/octet-stream' \
--header 'SSVM-Description: image recognition' \
--data-binary '@pkg/image_recognition_lib_bg.wasm'
```

Returns

```
{"wasm_id":123,"wasm_sha256":"0x...","usage_key":"00000...","admin_key":"00xxxxxx-xxxx-xxxx-xxxx-4adc960fd2b8"}
```

Make a function call via the web.

```
curl --location --request POST 'https://rpc.ssvm.secondstate.io:8081/api/run/123/infer' \
--header 'Content-Type: application/octet-stream' \
--data-binary '@test/grace_hopper.jpg'
```


