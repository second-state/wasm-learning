const { say, obfusticate, lowest_common_denominator, sha3_digest, keccak_digest } = require('./functions_lib.js');

var util = require('util');
const encoder = new util.TextEncoder();

console.log( say("SSVM") );
<!-- console.log( obfusticate("A quick brown fox jumps over the lazy dog") ); -->
console.log( lowest_common_denominator(123, 2) );
console.log( sha3_digest(encoder.encode("This is an important message")) );
console.log( keccak_digest(encoder.encode("This is an important message")) );

