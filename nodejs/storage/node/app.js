const { store_primitive_types, load_primitive_types, store_string, load_string, store_struct, load_struct, store_bytes, load_bytes } = require('../pkg/storage_lib.js');

// Utils
var util = require('util');
const encoder = new util.TextEncoder();
// From https://nielsleenheer.com/articles/2017/the-case-for-console-hex/
console.hex = (d) => console.log((Object(d).buffer instanceof ArrayBuffer ? new Uint8Array(d.buffer) : typeof d === 'string' ? (new util.TextEncoder('utf-8')).encode(d) : new Uint8ClampedArray(d)).reduce((p, c, i, a) => p + (i % 16 === 0 ? i.toString(16).padStart(6, 0) + '  ' : ' ') + c.toString(16).padStart(2, 0) + (i === a.length - 1 || i % 16 === 15 ?  ' '.repeat((15 - i % 16) * 3) + Array.from(a).splice(i - i % 16, 16).reduce((r, v) => r + (v > 31 && v < 127 || v > 159 ? String.fromCharCode(v) : '.'), '  ') + '\n' : ''), ''));

var sk = store_string("A quick brown fox jumps over the lazy dog");
console.log( load_string(sk) );

var sk = store_bytes(encoder.encode("A quick brown fox jumps over the lazy dog"));
console.hex( load_bytes(sk) );

var to_store = {a_vec:new Uint8Array([21, 31, 41]), a_i32:-42, a_u8:7, a_bool:true};
var sk = store_struct( JSON.stringify(to_store) );
var loaded = JSON.parse( load_struct(sk) );
console.log(loaded);

var to_store = [true, 's', 42, 3.14];
var sks = JSON.parse( store_primitive_types(JSON.stringify(to_store)) );
var loaded = JSON.parse( load_primitive_types(JSON.stringify(sks)) );
console.log(loaded);