# WASI standalone app

这个例子中，我们演示了如何从命令行运行一个独立的 WASM 应用程序。

## 先决条件

如果您还没有这样做，请按照这些简单的说明[安装 Rust、Node.js、WasmEdge 和 rustwasmc](https://www.secondstate.io/articles/setup-rust-nodejs/)。

## 构建 WASM 字节码

```
$ rustwasmc build --enable-aot
```

## 从命令行运行应用程序

我们将使用 WasmEdge 命令来运行程序

```
$ wasmedge --dir .:. pkg/wasi_example_main.wasm arg1 arg2
WASI standalone app


