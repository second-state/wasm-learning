import { get_random_i32, echo, print_args } from '../pkg/wasi_example_lib.js';

console.log( "My random number is: ", get_random_i32() );
echo("Hello Deno");
print_args();
