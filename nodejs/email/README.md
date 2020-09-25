# The email example

In this example, we will demonstrate how to redirect the return value from a FaaS function to another online service.

## Prerequisites

If you have not done so already, follow these simple instructions to install Node.js + SSVM, [Rust](https://www.rust-lang.org/tools/install) and [ssvmup](https://www.secondstate.io/articles/ssvmup/).

Install the `http_proxy` crate's binary executable on your local system.

```
$ cargo install http_proxy
```

## Build the WASM bytecode

```
$ ssvmup build
```

## Test

```
$ node node/test.js
```

