const fs = require('fs');
const { watermark } = require('../pkg/watermark_lib.js');

fs.writeFileSync("tmp-two.png", watermark("小胡", fs.readFileSync('template.png')));
fs.writeFileSync("tmp-three.png", watermark("胡晓维", fs.readFileSync('template.png')));
