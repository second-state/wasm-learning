# Face condition example

## Build

```bash
$ cargo build --release
```

The result binary will be at `target/release/face_detect`.
The tensorflow shared library will be at build path, such as
`target/release/build/tensorflow-sys-a8e272d10628f6f3/out/libtensorflow.so.1` and
`target/release/build/tensorflow-sys-a8e272d10628f6f3/out/libtensorflow_framework.so.1`.

If error occurs when building, try to update `rustup`.
```bash
$ rustup update nightly
$ rustup update stable
```
