# The regex example

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
--header 'SSVM-Description: regex match' \
--data-binary '@pkg/match_regex_lib_bg.wasm'
```

Returns

```
{"wasm_id":162,"wasm_sha256":"0xfb413547a8aba56d0349603a7989e269f3846245e51804932b3e02bc0be4b665","usage_key":"00000000-0000-0000-0000-000000000000","admin_key":"f9d45210-2e9b-49fa-bbb3-e2b111c22f46"}
```

Make a function call via the web.

```
curl --location --request POST 'https://rpc.ssvm.secondstate.io:8081/api/run/162/match_text' \
--header 'Content-Type: text/plain' \
--data '["\\d{4}-\\d{2}-\\d{2}","On 2009-01-03, Satoshi Nakamoto launched the Bitcoin blockchain. The price reached a high of $19,783.06 on 2017-12-17 and dropped to a low of $3,300 on 2018-12-07."]'
```

## Web test

Load this [static web page](html/index.html) to interact with the function. Look, serverless!

https://second-state.github.io/wasm-learning/faas/regex/html/index.html