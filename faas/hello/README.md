# The hello example

In this example, we demonstrate how to create and run a Rust function in the Second State Rust FaaS.

## Set up

[See it here](https://cloud.secondstate.io/function-as-a-service/getting-started).

## Write Rust code

Below is the entire content of the [src/lib.rs](src/lib.rs) file.

```
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn say(context: &str, s: &str) -> String {
  let r = String::from("hello ");
  return r + s;
}
```

## Build the WASM bytecode

```
$ ssvmup build --nowasi
```

## FaaS

Upload the wasm file to the FaaS.

```
$ curl --location --request POST 'https://rpc.ssvm.secondstate.io:8081/api/executables' \
--header 'Content-Type: application/octet-stream' \
--header 'SSVM-Description: say hello' \
--data-binary 'pkg/hello_lib_bg.wasm'
{"wasm_id":123}
```

Make a function call via the web.

```
$ curl --location --request POST 'https://rpc.ssvm.secondstate.io:8081/api/run/123/say' \
--header 'Content-Type: text/plain' \
--data-raw 'Second State FaaS'
hello Second State FaaS
```


