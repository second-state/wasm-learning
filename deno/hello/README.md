## Prerequisites

[Install Deno](https://deno.land/manual/getting_started/installation)

[Install Rust](https://www.rust-lang.org/tools/install)

## Setup

```
$ npm install -g ssvmup # Append --unsafe-perm if permission denied
```

## Build

```
$ ssvmup build --target deno
```

## Test

```
$ cd deno
$ deno run --allow-read test.ts
```

## Run

```
$ cd deno
$ deno run --allow-net --allow-read server.ts
```

## User test

```
$ curl http://localhost:3000/
hello World!
```
