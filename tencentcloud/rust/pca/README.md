# Rust getting started

See more here: https://doc.rust-lang.org/book/ch01-03-hello-cargo.html

## Install and setup

```
$ sudo apt-get update
$ sudo apt-get -y upgrade

$ curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
$ source $HOME/.cargo/env
$ rustup target add x86_64-unknown-linux-musl
```

## Compile and run

```
$ cargo build --target x86_64-unknown-linux-musl --release
    Finished release [optimized] target(s) in 0.22s
$ cp target/x86_64-unknown-linux-musl/release/pca cloud/
```

