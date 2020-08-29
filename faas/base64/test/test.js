const fs = require('fs');
const { encode, decode } = require('../pkg/base64_conv_lib.js');

var enc = encode(fs.readFileSync('one_pixel_png_image.png'));
console.log(enc);
fs.writeFileSync("tmp.png", decode(enc));
