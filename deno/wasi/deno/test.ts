import { get_random_i32, echo, print_env, create_file, read_file, del_file } from '../pkg/wasi_example_lib.js';

console.log( "My random number is: ", get_random_i32() );
echo("Hello Deno");
print_env();
create_file("/hello.txt", "Hello WASI");
console.log( read_file("/hello.txt") );
del_file("/hello.txt");
