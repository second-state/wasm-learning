# The callback example

In this example, we will demonstrate how to redirect the return value from a FaaS function to another online service.

## Prerequisites

If you have not done so already, follow these simple instructions to install [Rust](https://www.rust-lang.org/tools/install) and [ssvmup](https://www.secondstate.io/articles/ssvmup/).

## Create a new Rust project
```
cargo new --lib hello_callback
```
Edit the config by appending the following `[lib]` and `[dependencies]` to the end of the `Cargo.toml` file (after the `[packages]` section)
```
[lib]
crate-type = ["cdylib", "rlib"]

[dependencies]
wasm-bindgen = "=0.2.61"
serde_json = "1.0"
```

## Write Rust code

Below is the entire content of the [src/lib.rs](src/lib.rs) file.

```
use wasm_bindgen::prelude::*;
use serde_json::json;

#[wasm_bindgen]
pub fn say(s: &str) -> String {
  let r = String::from("hello ");
  // Create a JSON object which sendgrid expects
  let ret = json!(
    {
        "personalizations": [{
            "to": [{
                "email": "juntao_yuan@yahoo.com"
            }]
        }],
        "from": {
            "email": "michael@secondstate.io"
        },
        "subject":&(r + &s),
        "content": [{
            "type": "text/plain",
            "value": "This is a message from Joey and SSVM"
        }]
    });
  return ret.to_string();
}
```

## Build the WASM bytecode

```
$ ssvmup build
```

## Create FaaS function

Upload the wasm file to the FaaS.

```
$ curl --location --request POST 'https://rpc.ssvm.secondstate.io:8081/api/executables' \
--header 'Content-Type: application/octet-stream' \
--header 'SSVM-Description: say hello' \
--data-binary '@pkg/hello_callback_bg.wasm'
```
Returns
```
{"wasm_id":123,"wasm_sha256":"0xec9e4c7d01920f...644bed9bf7922","SSVM_Usage_Key":"00000000-0000-0000-0000-000000000000","SSVM_Admin_Key":"b425089...8bfa58e6"}
```

## Redirect results to another service

This is done by associating a callback object with the wasm file. The callback is a HTTP request object in JSON format. The function call's return value is submiited to the callback upon completion.

```
curl --location --request PUT 'https://rpc.ssvm.secondstate.io:8081/api/callback/123' \
--header 'Content-Type: application/json' \
--data-raw '{"hostname": "api.sendgrid.com","path": "/v3/mail/send","method": "POST","port": 443,"headers":{"Content-Type": "application/json","authorization": "Bearer SG.xxxx"}}'
```

## Test

Make a function call via the web.

```
$ curl --location --request POST 'https://rpc.ssvm.secondstate.io:8081/api/run/123/say' \
--header 'Content-Type: text/plain' \
--data-raw 'Second State FaaS'
```

The `TO_EMAIL` address in the function will now receive an email message with the "hello Second State FaaS" subject line.




