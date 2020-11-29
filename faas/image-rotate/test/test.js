const fs = require('fs');
const { watermark } = require('../pkg/watermark_lib.js');

fs.writeFileSync("tmp-cat.png", watermark("Meow world!", fs.readFileSync('cat.png')));
fs.writeFileSync("tmp-dog.png", watermark("Woof world!", fs.readFileSync('dog.png')));
