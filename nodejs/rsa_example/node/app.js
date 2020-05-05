const { generate_key_pair, encrypt, decrypt } = require('../pkg/rsa_example_lib.js');

var util = require('util');
const encoder = new util.TextEncoder();
const decoder = new util.TextDecoder();

console.time('generate_key_pair');
var kp = JSON.parse(generate_key_pair(2048));
console.timeEnd('generate_key_pair');
var public_key = kp['rsa_public_key'];
var private_key = kp['rsa_private_key'];

var msg = "The Times 03/Jan/2009 Chancellor on brink of second bailout for banks";
console.time('encrypt_decrypt');
var enc_data = encrypt(JSON.stringify(public_key), encoder.encode(msg));
var dec_data = decrypt(JSON.stringify(private_key), enc_data);
console.timeEnd('encrypt_decrypt');
console.log(decoder.decode(dec_data));
