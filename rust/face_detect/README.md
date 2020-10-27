# Face condition example

## Build

```bash
$ cargo build --release
```
The result binary will be at `target/release/face_detect`.

If error occurs when building, try to update `rustup`.
```bash
$ rustup update nightly
$ rustup update stable
```

## Install the Tensorflow Library

[Follow the instructions](https://www.tensorflow.org/install/lang_c). On most Linux systems, just do the following.

```bash
$ wget https://storage.googleapis.com/tensorflow/libtensorflow/libtensorflow-gpu-linux-x86_64-1.15.0.tar.gz
$ sudo tar -C /usr/local -xzf libtensorflow-gpu-linux-x86_64-1.15.0.tar.gz
$ sudo ldconfig
```
