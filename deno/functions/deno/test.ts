import { say, obfusticate, lowest_common_denominator, sha3_digest, keccak_digest, create_line } from '../pkg/functions_lib.js';

const encoder = new TextEncoder();

console.log( say("SSVM") );
console.log( obfusticate("A quick brown fox jumps over the lazy dog") );
console.log( lowest_common_denominator(123, 2) );
console.log( sha3_digest(encoder.encode("This is an important message")) );
console.log( keccak_digest(encoder.encode("This is an important message")) );

var p1 = {x:1.5, y:3.8};
var p2 = {x:2.5, y:5.8};
var line = JSON.parse(create_line(JSON.stringify(p1), JSON.stringify(p2), "A thin red line"));
console.log( line );
