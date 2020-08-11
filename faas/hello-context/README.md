# The hello example

In this example, we demonstrate how to create and run a Rust function in the Second State Rust FaaS.

# The end in mind
If you want to try out this feature before writing the rust code, please use the Curl examples which are available at the bottom of this page.

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

Before compiling, be sure to append the following to your `Cargo.toml` file (after the `[package]` section)
```
[lib]
crate-type = ["cdylib", "rlib"]

[dependencies]
wasm-bindgen = "=0.2.61"
```

## Build the WASM bytecode

```
$ ssvmup build
```

## FaaS

Upload the wasm file to the FaaS. Perhaps have a quick check to see what the name of the file in the `pkg` dir is i.e. `hello_bg.wasm` or `hello_lib_bg.wasm etc.

```
$ curl --location --request POST 'https://rpc.ssvm.secondstate.io:8081/api/executables' \
--header 'Content-Type: application/octet-stream' \
--header 'SSVM-Description: say hello' \
--data-binary '@pkg/hello_bg.wasm'
```
If you are deploying your own Wasm you will get a fresh wasm id for you to use on the upcoming requests. wasm_id 69 is one we prepared earlier.
```
{"wasm_id":69}
```

Set the environmental variable `EMOJI` for functions in this wasm file.

```
$ curl --location --request PUT 'https://rpc.ssvm.secondstate.io:8081/api/state/69' --header 'Content-Type: text/plain' --data-raw 'true'
```

Make a function call via the web.

```
$ curl --location --request POST 'https://rpc.ssvm.secondstate.io:8081/api/run/69/say' \
--header 'Content-Type: text/plain' \
--data-raw 'Second State FaaS'
```
The following answer is returned from the previous function call
```
👋 Second State FaaS
```
Now if we set the environment variable to false
```
curl --location --request PUT 'https://rpc.ssvm.secondstate.io:8081/api/state/69' \
--header 'Content-Type: text/plain' \
--data-raw 'false'
```
We will get the non-emoji response if we again call that function
```
$ curl --location --request POST 'https://rpc.ssvm.secondstate.io:8081/api/run/69/say' \
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
curl https://rpc.ssvm.secondstate.io:8081/api/state/69
```


