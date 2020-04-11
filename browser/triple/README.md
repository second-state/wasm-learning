
# The triple example

This is a bare bone example to show how WASM programs interact with browser-based JS host. The JS host calls the WASM program to perform computations.


## Set up

```
$ sudo apt-get update
$ sudo apt-get -y upgrade

$ sudo apt-get -y install apache2
$ sudo chown -R $USER:$USER /var/www/html
$ sudo systemctl start apache2

$ curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
$ source $HOME/.cargo/env
$ rustup target add wasm32-wasi
$ rustup override set nightly
$ rustup target add wasm32-wasi --toolchain nightly
```

## Create new project

```
$ cargo new --lib triple
$ cd triple
```

## Change the cargo config file

Add the following to the [Cargo.toml](Cargo.toml) file.

```
[lib]
name = "triple_lib"
path = "src/lib.rs"
crate-type =["cdylib"]
```

## Write Rust code

Below is the entire content of the [src/lib.rs](src/lib.rs) file.

```
#[no_mangle]
pub extern fn triple(x: i32) -> i32 {
  return 3 * x;
}
```

## Build the WASM bytecode

```
$ cargo +nightly build --target wasm32-wasi --release
```

## Create a new HTML folder

```
$ mkdir html
$ cp target/wasm32-wasi/release/triple_lib.wasm html/
```

## Create an html file

Below is the content of the [html/index.html](html/index.html) file.

```
<html>
  <head>
    <link rel="stylesheet" href="https://stackpath.bootstrapcdn.com/bootstrap/4.3.1/css/bootstrap.min.css" integrity="sha384-ggOyR0iXCbMQv3Xipma34MD+dH/1fQ784/j6cY/iJTQUOhcWr7x9JvoRxT2MZw1T" crossorigin="anonymous" />
    <script>
      if (!('WebAssembly' in window)) {
        alert('you need a browser with wasm support enabled :(');
      }
      (async () => {
      const response = await fetch('triple_lib.wasm');
      const buffer = await response.arrayBuffer();
      const module = await WebAssembly.compile(buffer);
      const instance = await WebAssembly.instantiate(module);
      const exports = instance.exports;
      const triple = exports.triple;
      var buttonOne = document.getElementById('buttonOne');
          buttonOne.value = 'Triple the number';
          buttonOne.addEventListener('click', function() {
            var input = $("#numberInput").val();
            alert(input + ' tripled equals ' + triple(input));
          }, false);
      })();    
      
    </script>
  </head>
  <body>
    <div class="row">
      <div class="col-sm-4"></div>
      <div class="col-sm-4">
        <b>Wasm - Triple the number</b>
      </div>
      <div class="col-sm-4"></div>
    </div>
    <hr />
    <div class="row">
      <div class="col-sm-2"></div>
      <div class="col-sm-4">Place a number in the box</div>
      <div class="col-sm-4"> Click the button</div>
      <div class="col-sm-2"></div>
    </div>
    <div class="row">
      <div class="col-sm-2"></div>
      <div class="col-sm-4">
        <input type="text" id="numberInput" placeholder="1", value="1">
      </div>
      <div class="col-sm-4">
        <button class="bg-light" id="buttonOne">Triple the number</button>
      </div>
      <div class="col-sm-2"></div>
    </div>
  </body>
  <script
    src="https://code.jquery.com/jquery-3.4.1.js"
    integrity="sha256-WpOohJOqMqqyKL9FccASB9O0KwACQJpFTUBLTYOVvVU="
    crossorigin="anonymous"></script>
</html>
```

## Deploy the static files to Apache

```
$ cp html/* /var/www/html/
```

## Test

```
http://100.24.46.159/
```


