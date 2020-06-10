
Setup

```
$ npm install -g ssvmup # Append --unsafe-perm if permission denied
```

Build

```
$ ssvmup build --target deno
```

Run

```
$ deno run --allow-net --allow-read server.ts
```

Test

```
$ curl http://localhost:8000/
hello World!
```
