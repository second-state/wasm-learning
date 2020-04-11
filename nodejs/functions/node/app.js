const { say, obfusticate, lowest_common_denominator, sha3_digest, keccak_digest, create_line } = require('./functions_lib.js');

// Utils
var util = require('util');
const encoder = new util.TextEncoder();
// From https://nielsleenheer.com/articles/2017/the-case-for-console-hex/
console.hex = (d) => console.log((Object(d).buffer instanceof ArrayBuffer ? new Uint8Array(d.buffer) : typeof d === 'string' ? (new util.TextEncoder('utf-8')).encode(d) : new Uint8ClampedArray(d)).reduce((p, c, i, a) => p + (i % 16 === 0 ? i.toString(16).padStart(6, 0) + '  ' : ' ') + c.toString(16).padStart(2, 0) + (i === a.length - 1 || i % 16 === 15 ?  ' '.repeat((15 - i % 16) * 3) + Array.from(a).splice(i - i % 16, 16).reduce((r, v) => r + (v > 31 && v < 127 || v > 159 ? String.fromCharCode(v) : '.'), '  ') + '\n' : ''), ''));

console.log( say("SSVM") );
console.log( obfusticate("A quick brown fox jumps over the lazy dog") );
console.log( lowest_common_denominator(123, 2) );
console.hex( sha3_digest(encoder.encode("This is an important message")) );
console.hex( keccak_digest(encoder.encode("This is an important message")) );

var p1 = {x:1.5, y:3.8};
var p2 = {x:2.5, y:5.8};
var line = JSON.parse(create_line(JSON.stringify(p1), JSON.stringify(p2), "A thin red line"));
console.log( line );
