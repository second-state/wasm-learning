const { get_random_i32, get_random_bytes, print_random_i32, print_env, create_file } = require('../pkg/wasi_example_lib.js');

// Utils
var util = require('util');
const encoder = new util.TextEncoder();
// From https://nielsleenheer.com/articles/2017/the-case-for-console-hex/
console.hex = (d) => console.log((Object(d).buffer instanceof ArrayBuffer ? new Uint8Array(d.buffer) : typeof d === 'string' ? (new util.TextEncoder('utf-8')).encode(d) : new Uint8ClampedArray(d)).reduce((p, c, i, a) => p + (i % 16 === 0 ? i.toString(16).padStart(6, 0) + '  ' : ' ') + c.toString(16).padStart(2, 0) + (i === a.length - 1 || i % 16 === 15 ?  ' '.repeat((15 - i % 16) * 3) + Array.from(a).splice(i - i % 16, 16).reduce((r, v) => r + (v > 31 && v < 127 || v > 159 ? String.fromCharCode(v) : '.'), '  ') + '\n' : ''), ''));

console.log( "My random number is: ", get_random_i32() );
console.log( "My random bytes are");
console.hex( get_random_bytes() );
print_random_i32();
print_env();
create_file("hello.txt", "Hello WASI SSVM");
