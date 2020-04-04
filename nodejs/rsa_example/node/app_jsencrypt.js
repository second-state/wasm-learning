const JSEncrypt = require('node-jsencrypt');

console.time("generate_keys");
var crypt = new JSEncrypt({ default_key_size: 2048 });
console.timeEnd("generate_keys");
var public_key = crypt.getPublicKey();
var private_key = crypt.getPrivateKey();


var msg = "The Times 03/Jan/2009 Chancellor on brink of second bailout for banks";

console.time("encrypt_decrypt");
// for (var i = 0; i < 1000; i++) {
  crypt = new JSEncrypt();
  crypt.setPublicKey(public_key);
  var enc = crypt.encrypt(msg);

  crypt = new JSEncrypt();
  crypt.setPrivateKey(private_key);
  var dec = crypt.decrypt(enc);
// }
console.timeEnd("encrypt_decrypt");

console.log(dec);
