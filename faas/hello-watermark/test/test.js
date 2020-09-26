const fs = require('fs');
const { watermark } = require('../pkg/hello_watermark_lib.js');

fs.writeFileSync("tmp.png", watermark(fs.readFileSync('cat.png')));
