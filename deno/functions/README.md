
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
$ deno run --allow-read test.ts
```
