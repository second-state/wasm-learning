# The SSD example

Run SSD tensorflow models as functions.



## Prerequisites

If you have not done so already, follow these simple instructions to install [Rust](https://www.rust-lang.org/tools/install) and [ssvmup](https://www.secondstate.io/articles/ssvmup/).
new model here https://drive.google.com/file/d/1YN4nYVbA0hMtPZDZlIlN5M9Z8qllHVM-/view?usp=sharing download through this link. 


## Build the WASM bytecode

```
$ ssvmup build --enable-aot --enable-ext
```
## Create FaaS function

Upload the wasm file in the `pkg` folder to the FaaS. Double check the `.wasm` file name before you upload.

```
$ curl --location --request POST 'https://rpc.ssvm.secondstate.io:8081/api/executables' \
--header 'Content-Type: application/octet-stream' \
--header 'SSVM-Description: ssd' \
--data-binary '@pkg/ssd_service_lib_bg.wasm'
```

Returns

```
{"wasm_id":your_wasmid,"wasm_sha256":"0x469c28daae7aba392076b4bc5ee3b43ec6d667083d8ae63207bf74b1da03fc26","SSVM_Usage_Key":"00000000-0000-0000-0000-000000000000","SSVM_Admin_Key":"7dxxxxxx-xxxx-xxxx-xxxx-xxxxxxxx0c41"}
```

Note: You can update this binary with the `SSVM_Admin_Key`.

```
$ curl --location --request PUT 'https://rpc.ssvm.secondstate.io:8081/api/update_wasm_binary/your_wasmid' \
--header 'Content-Type: application/octet-stream' \
--header 'SSVM_Admin_Key: 7dxxxxxx-xxxx-xxxx-xxxx-xxxxxxxx0c41' \
--data-binary '@pkg/ssd_service_lib_bg.wasm'
```

## Test

Make a function call via the web.

```
$ curl --location --request POST 'https://rpc.ssvm.secondstate.io:8081/api/run/your_wasmid/infer/bytes' \
--header 'Content-Type: application/octet-stream' \
--data-binary '@src/ride.png' \
--output result.jpg
```
image.jpg is your own custom image, not upload here.
