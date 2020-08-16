# Permanent storage example (immutable)

If you would like to try this out, please just compile the pre-written demonstration using the following instructions

If you haven't done so already. Please go ahead and install [Rust](https://www.rust-lang.org/tools/install) and [ssvmup](https://www.secondstate.io/articles/ssvmup/).

## Build
Clone this `wasm-learning` repository and then, from this `hello-storage` directory (where you are reading this README file), run the following command

```bash
ssvmup build
```

## Launch
Once you have compiled the demonstration, go ahead and launch the Wasm executable using the following Curl command.

```bash
curl --location --request POST 'https://rpc.ssvm.secondstate.io:8081/api/executables' --header 'Content-Type: application/octet-stream' --header 'SSVM-Description: storage' --data-binary '@pkg/hello_storage_bg.wasm'
```

The above command will return a JSON string. Inside the JSON will be the `wasm_id` that you need to execute the store and load functions.

```bash
{"wasm_id":123, ...  }
```

## Store
The following command will execute the `store_a_string` function and store the text that we are passing into the command i.e. `String to store`.

Be sure to use your own `wasm_id` from above (not `123`)

```bash
curl --location --request POST 'https://rpc.ssvm.secondstate.io:8081/api/run/123/store_a_string' \
--header 'Content-Type: text/plain' \
--data-raw 'String to store'
```

The above command will return a key like the one shown below.
```bash
29cc8944469e42189497a72b4a8ad737
```
## Load
We can use this key to fetch the `String to store` data which we stored earlier. Here is an example of the HTTP request to achieve this.

```bash
curl --location --request POST 'https://rpc.ssvm.secondstate.io:8081/api/run/123/load_a_string' \
--header 'Content-Type: text/plain' \
--data-raw '29cc8944469e42189497a72b4a8ad737'
```

The command above returns the original string like this

```
String to store
```

# Permanent storage example (mutable)

You will notice with the above examples that a brand new storage key is freshly minted each time any data is stored. There will be times that you might prefer to use the same storage key and update the data over and over. The following example shows you how to access a data storage key which we have baked into your Wasm deployment for your convenience.

When you launch your Wasm file (as shown above) a permanent storage key is created and held for your future use. Interestingly, we make this key available inside your Rust/Wasm code (by passing it into std::env for your convenience). What this means is that you don't even need to know the key. You just need to know that there is a key available for you inside your Rust code at the following location `env::var("storage_key")`. If you would like to store a value to the location of that key you can simply use the following syntax in your Rust/Wasm
```
let _string_to_store = String::from("A string to store!");
let storage_key: String = env::var("storage_key").unwrap();
ssvm_storage::store::update(&storage_key, _string_to_store);
```
If you would like to access the data stored at that permanent location, you can use the following syntax
```
let storage_key: String = env::var("storage_key").unwrap();
let retrieved_string: String = ssvm_storage::load::load_as_string(&storage_key);
```
Seeings how you have lauched the Wasm in this demo already, let's go ahead and call the `store_a_string_via_std_env` and `load_a_string_via_std_env` functions that we prepared earlier.

## Store
The following call will store the raw data at the mutable storage location for your wasm_id
```bash
curl --location --request POST 'https://dev.rpc.ssvm.secondstate.io:8081/api/run/wasm_id/store_a_string_via_std_env' \
--header 'Content-Type: text/plain' \
--data-raw 'This is a string to store'
```

## Load
The following call will load the raw data at the mutable storage location for your wasm_id
```bash
curl --location --request POST 'https://dev.rpc.ssvm.secondstate.io:8081/api/run/wasm_id/load_a_string_via_std_env'
```



