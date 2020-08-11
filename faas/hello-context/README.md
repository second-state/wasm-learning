# The hello example

In this example, we demonstrate how to create and run a Rust function in the Second State Rust FaaS.

## Prerequisites

If you have not done so already, follow these simple instructions to install [Rust](https://www.rust-lang.org/tools/install) and [ssvmup](https://www.secondstate.io/articles/ssvmup/).

## Write Rust code

Below is the entire content of the [src/lib.rs](src/lib.rs) file.

```
use std::env;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn say(s: &str) -> String {
    // Access arguments from std env
    let arguments: Vec<String> = env::args().collect();
    // Convert argument in position 1 to boolean
    let use_emoji_bool: bool = arguments[1].parse().unwrap();
    if use_emoji_bool {
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
--data-binary '@pkg/hello_lib_bg.wasm'
{"wasm_id":123}
```

Set the environmental variable `EMOJI` for functions in this wasm file.

```
$ curl --location --request PUT 'https://rpc.ssvm.secondstate.io:8081/api/state/123' --header 'Content-Type: text/plain' --data-raw 'true'
```

Make a function call via the web.

```
$ curl --location --request POST 'https://rpc.ssvm.secondstate.io:8081/api/run/123/say' \
--header 'Content-Type: text/plain' \
--data-raw 'Second State FaaS'
```
The following answer is returned from the previous function call
```
👋 Second State FaaS
```
Now if we set the environment variable to false
```
curl --location --request PUT 'https://dev.rpc.ssvm.secondstate.io:8081/api/state/123' \
--header 'Content-Type: text/plain' \
--data-raw 'false'
```
We will get the non-emoji response if we again call that function
```
$ curl --location --request POST 'https://rpc.ssvm.secondstate.io:8081/api/run/123/say' \
--header 'Content-Type: text/plain' \
--data-raw 'Second State FaaS'
```
Returns 
```
hello Second State FaaS
```
**Please note**
If you want to inspect the value stored in the environment variable (to know if you need to updated it), you can use the following GET request
```
curl https://dev.rpc.ssvm.secondstate.io:8081/api/state/123
```


