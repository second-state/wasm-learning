
## Setup

[Install Deno](https://deno.land/manual/getting_started/installation) first.

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
$ curl http://localhost:8000/
hello World!
```
