# Rust getting started

See more here: https://doc.rust-lang.org/book/ch01-03-hello-cargo.html

## Install and setup

```
$ sudo apt-get update
$ sudo apt-get -y upgrade

$ curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
$ source $HOME/.cargo/env
```

## Compile and run

```
$ cargo new hello
     Created binary (application) `hello` package
$ cd hello
$ cargo build --release
   Compiling hello v0.1.0 (/home/ubuntu/wasm-learning/rust/hello)
    Finished release [optimized] target(s) in 0.22s
$ target/release/hello
Hello, world!
```

