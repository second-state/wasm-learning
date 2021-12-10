# Reactr getting started

[Reactr](https://github.com/suborbital/reactr) is a fast, performant function scheduling library. Reactr is designed to be flexible, with the ability to run embedded in your Go applications and first-class support for WebAssembly.

Reactr runs functions called Runnables, and transparently spawns workers to process jobs. Each worker processes jobs in sequence, using Runnables to execute them. Reactr jobs are arbitrary data, and they return arbitrary data (or an error). Jobs are scheduled, and their results can be retrieved at a later time.

Reactr has support for Wasm-packaged Runnables. The rwasm package contains a multi-tenant Wasm scheduler, an API to grant capabilities to Wasm Runnables, and support for several languages including Rust (stable), TypeScript/AssemblyScript (beta), and Swift (alpha).

The default Wasm VM is Wasmer, WasmEdge can be turned on by passing `-tags wasmedge` to any go command.

## Install and setup Rust

```
$ sudo apt-get update
$ sudo apt-get -y upgrade

$ curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
$ source $HOME/.cargo/env
$ rustup target add wasm32-wasi
```

## Install and setup PostgreSQL
```
# Start PostgreSQL with Docker
$ docker pull postgres
$ docker run --name reactr-postgres -p 5432:5432 -e POSTGRES_PASSWORD=12345 -d postgres

# Attach via psql
$ docker run -it --rm --network host postgres psql -h 127.0.0.1 -U postgres

# Create a database
postgres=# CREATE DATABASE reactr;
postgres=# \c reactr;

# Create a table:
postgres=# CREATE TABLE users (
    uuid        varchar(100) CONSTRAINT firstkey PRIMARY KEY,
    email       varchar(50) NOT NULL,
    created_at  date,
    state       char(1),
    identifier  integer
);

# Export environment variable of connection url
$ export REACTR_DB_CONN_STRING='postgresql://postgres:12345@127.0.0.1:5432/reactr'
```

## Compile and run

```
$ cd rs-db
$ cargo build --target wasm32-wasi --release
$ cp target/wasm32-wasi/release/rs_db.wasm ..

$ cd ..
$ go mod tidy
$ go run main.go -tags wasmedge
```

