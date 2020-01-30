# Rust getting started

## Install and setup

```
$ sudo apt-get update
$ sudo apt-get -y upgrade

$ curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
$ source $HOME/.cargo/env
```

## Create project

```
$ cargo new cli
     Created binary (application) `cli` package
$ cd cli
```

## Write Rust code

Below is the content of the [src/main.rs](cli/src/main.rs) file.

```
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{}", String::from("Hello ") + &args[1]);
}
```

## Compile and run

```
$ cargo build --release
$ target/release/cli Rust
Hello Rust
```

