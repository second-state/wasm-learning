import { get_random_i32, get_random_bytes, echo, print_env, create_file } from '../pkg/wasi_example_lib.js';

const encoder = new TextEncoder();

echo("Hello Deno");
print_env();
// create_file("hello.txt", "Hello WASI");
