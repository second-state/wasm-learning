# The Node.js API example

In this example, we demonstrate how to call Node.js functions from Rust applications in SSVM.

Build the app as follows.

```
$ wasm-pack build --target nodejs
```

Run the example as follows.

```
$ cd node
$ node app.js
Read file: 5.050ms
Resize: 1343.333ms
Write file: 0.698ms
```

Now you should see a new `cat-100-100.png` file in the `node` directory.
