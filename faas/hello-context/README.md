# The hello example

In this example, we demonstrate how to create and run a Rust function in the Second State Rust FaaS.

## Prerequisites

If you have not done so already, follow these simple instructions to install [Rust](https://www.rust-lang.org/tools/install) and [ssvmup](https://www.secondstate.io/articles/ssvmup/).

## Write Rust code

Below is the entire content of the [src/lib.rs](src/lib.rs) file.

```
use wasm_bindgen::prelude::*;
use std::env;

#[wasm_bindgen]
pub fn say(s: &str) -> String {
  let emoji = env::var("EMOJI").unwrap_or_default(false);
  if emoji {
    let r = String::from("👋 ");
    return r + &s;
  } else {
    let r = String::from("hello ");
    return r + &s;
  }
}
```

## Build the WASM bytecode

```
$ ssvmup build
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

Set the environmental variable `EMOJI` for functions in this wasm file.

```
TBD
```

Make a function call via the web.

```
$ curl --location --request POST 'https://rpc.ssvm.secondstate.io:8081/api/run/123/say' \
--header 'Content-Type: text/plain' \
--data-raw 'Second State FaaS'
👋 Second State FaaS
```


