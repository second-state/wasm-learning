# The Node.js API example

In this example, we demonstrate how to call Node.js functions from Rust applications in SSVM. It works like this.

* Node.js apps call Rust functions running in WebAssembly (e.g., from the SSVM). They do it for performance, security, and code portability reasons. Checkout [why](https://www.secondstate.io/articles/why-webassembly-server/) and [how](https://www.secondstate.io/articles/getting-started-with-rust-function/).
* The Rust function occasionally needs to access system recources such as the current time, printing to the console, the file system, and / or the network. It does so using the [`nodejs-helper`](https://crates.io/crates/nodejs-helper) crate here.

## Prerequisite

Must have Node.js installed with the following packages.

```
$ npm i ssvm sync-request better-sqlite3
$ npm i -g ssvmup
$ npm i -g wasm-pack
```

## Build the demo app

The Rust functions that make Node.js calls are in the `src/lib.rs` file.

```
$ wasm-pack build --target nodejs
```

## Run the examples

The JavaScript files in the `node` directory simply calls the Rust functions in `src/lib.rs`, which in turns calls Node.js APIs to access system resources.

```
$ cd node
```

The first example is to access the system clock and standard output console from Rust functions running inside the WebAssembly VM.

```
$ node date.js
Timestamp now:
1588013800826
UTC time:
Mon, 27 Apr 2020 18:56:40 GMT
America/Chicago
Monday, April 27, 2020, CDT
```

Next, let's see how to create, update, and query a SQLite database from the Rust function running in WebAssembly.

```
$ node db.js
1 : Bob McFett
2 : Angus Vader
3 : Imperator Colin
```

From WebAssembly, we can also make HTTP requests, and save fetched content to a local file.

```
$ node http.js
https://raw.githubusercontent.com/second-state/nodejs-helper/master/LICENSE
MIT License

Copyright (c) 2020 Second State

Permission is hereby granted, free of charge, to any person obtaining a copy
... ...
```

We can use the Rust `image` crate to resize a PNG image from the local file system. We can also use Javascript `console` functions to profile performance in Rust.

```
$ node image.js
Resize file: 5.603ms Done reading
Resize file: 1506.694ms Done resizing
Resize file: 1507.634ms Done writing
Resize file: 1507.977ms
```

Finally, clean up all the temp files we have written to the file system.

```
$ node cleanup.js
```

To see more detailed explanations on those examples, [check out our tutorial article](https://www.secondstate.io/articles/rust-functions-in-nodejs/).

