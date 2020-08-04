
Check out this tutorial article: *[Deno Apps with WebAssembly, Rust, and WASI](https://www.secondstate.io/articles/deno-webassembly-rust-wasi/)*

## Prerequisites

[Install Deno](https://deno.land/manual/getting_started/installation)

[Install Rust](https://www.rust-lang.org/tools/install)

Also, if you have not done so, please bring your Linux system up to date with dev tools.

```
sudo apt install build-essential curl wget git vim libboost-all-dev
```

## Setup

```
$ npm install -g ssvmup # Append --unsafe-perm if permission denied
```

## Build

```
$ ssvmup build --target deno
```

## Test library

```
$ deno run --allow-read --allow-env --unstable deno/test.ts arg1 arg2
```

## Test main exec

```
$ deno run --allow-read --allow-write --allow-env --unstable pkg/deno_wasi_example.js
```
