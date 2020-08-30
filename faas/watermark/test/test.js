const fs = require('fs');
const { watermark } = require('../pkg/watermark_lib.js');

fs.writeFileSync("tmp.png", watermark(fs.readFileSync('cat.png')));
