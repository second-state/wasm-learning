# The email example

In this example, we will demonstrate how to redirect the return value from a FaaS function to another online service.

## Prerequisites

If you have not done so already, follow these simple instructions to install [Rust](https://www.rust-lang.org/tools/install) and [rustwasmc](https://www.secondstate.io/articles/ssvmup/).

## Build the WASM bytecode

```
rustwasmc build
```

## Create FaaS function

Upload the wasm file in the `pkg` folder to the FaaS. Double check the `.wasm` file name before you upload.

```
curl --location --request POST 'https://rpc.ssvm.secondstate.io:8081/api/executables' \
--header 'Content-Type: application/octet-stream' \
--header 'SSVM-Description: send email' \
--data-binary '@pkg/send_email_lib_bg.wasm'
```

Returns

```
{"wasm_id":481,"wasm_sha256":"0xd8d98d8edbd445c97e663c68d2067c9528fbd9bbfbf8a3ad39a5ba3f88b9cd34","SSVM_Usage_Key":"00000000-0000-0000-0000-000000000000","SSVM_Admin_Key":"f958eebd-c681-4f39-b0d1-83d894548c1d"}%  
```

Note: You can update this binary with the `SSVM_Admin_Key`.

```
$ curl --location --request PUT 'https://rpc.ssvm.secondstate.io:8081/api/update_wasm_binary/481' \
--header 'Content-Type: application/octet-stream' \
--header 'SSVM_Admin_Key: 7dxxxxxx-xxxx-xxxx-xxxx-xxxxxxxx0c41' \
--data-binary '@pkg/send_email_lib_bg.wasm'
```

## Test

Make a function call via the web.

```
curl --location --request POST 'https://rpc.ssvm.secondstate.io:8081/api/run/481/send_email' \
--header 'Content-Type: text/plain' \
--data '{"from":"michael@secondstate.io", "token":"SG.xxx", "to":"juntao_yuan@yahoo.com", "subject":"This is a HTTP Proxy FaaS test", "mime":"text/plain", "body":"Hello Second State FaaS!"}'
```

