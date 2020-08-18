# Cumulative storage example

## In a hurry?
**Here is one we prepared earlier.**
If you want to see the Wasm incrementer in action, please just run the following curl command
## Curl
```bash
curl --location --request POST 'https://rpc.ssvm.secondstate.io:8081/api/run/46/increment'
```
or, execute the following Javascript (in your web browser's console)
## Javascript
```Javascript
var requestOptions = {
  method: 'POST',
  redirect: 'follow'
};

fetch("https://rpc.ssvm.secondstate.io:8081/api/run/46/increment", requestOptions)
  .then(response => response.text())
  .then(result => console.log(result))
  .catch(error => console.log('error', error));
```
As you can see, these are just HTTP requests. So feel free to use any language you want i.e. Python, Go, Node.js, PHP etc.

## Under the hood
If you would like to try this out, please just compile the pre-written demonstration using the following instructions

If you haven't done so already. Please go ahead and install [Rust](https://www.rust-lang.org/tools/install) and [ssvmup](https://www.secondstate.io/articles/ssvmup/).

Clone this `wasm-learning` repository and then, from this `cumulative-storage` directory (where you are reading this README file), run the following command

```bash
ssvmup build
```

Once you have compiled the demonstration, go ahead and launch the Wasm executable using the following Curl command.

```bash
curl --location --request POST 'https://rpc.ssvm.secondstate.io:8081/api/executables' --header 'Content-Type: application/octet-stream' --header 'SSVM-Description: cumulative demo' --data-binary '@pkg/cumulative_storage_bg.wasm'
```

The above command will return a JSON string. Inside the JSON will be the `wasm_id` that you need to execute the store and load functions.

```bash
{"wasm_id":123, ...  }
```

## Initialize a starting value - mandatory

The following command will execute the `init` function which will pass an arbitrary value to kick things off i.e. `1` or `1000`

Be sure to use your own `wasm_id` from above (not `123`)

```bash
curl --location --request POST 'https://rpc.ssvm.secondstate.io:8081/api/run/your_wasm_id_goes_here/init' \
--header 'Content-Type: text/plain' \
--data-raw '1000'
```

## Start incrementing your starting value

Once you have initialized the Wasm, as shown above, then go ahead and increment your value. You can do this as many times as you like. The value will be updated and remain persistent for when you next return to update/increment it again.

```bash
curl --location --request POST 'https://rpc.ssvm.secondstate.io:8081/api/run/your_wasm_id_goes_here/increment'
```
