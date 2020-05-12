const { create_identity, create_plain_text, encrypt, create_transform_key, transform, decrypt } = require('../pkg/recrypt_example_lib.js');

var org_identity = JSON.parse(create_identity());
var dst_identity = JSON.parse(create_identity());
// console.log(org_identity);

var plain_text = JSON.parse(create_plain_text());
console.log(plain_text);

var args = [plain_text, org_identity['public_key_x'], org_identity['public_key_y'], org_identity['signing_key_pair']];
var ev = JSON.parse( encrypt(JSON.stringify(args)) );
// console.log("EV");
// console.log(ev);

var args = [org_identity['private_key'], dst_identity['public_key_x'], dst_identity['public_key_y'], org_identity['signing_key_pair']];
var tk = JSON.parse( create_transform_key(JSON.stringify(args)) );
// console.log("TK");
// console.log(tk);

var args = [ev, tk, org_identity['signing_key_pair']];
var tv = JSON.parse( transform(JSON.stringify(args)) );
// console.log("TV");
// console.log(tv);

var args = [tv, dst_identity['private_key']];
var pt = JSON.parse( decrypt(JSON.stringify(args)) );
console.log("PT");
console.log(pt);

