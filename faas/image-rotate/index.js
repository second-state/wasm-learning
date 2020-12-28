const { rotate_an_image } = require("./pkg/rotate_lib.js");

const fs = require('fs');
var data_img_hopper = fs.readFileSync("html/lean.png");

fs.writeFileSync("tmp.png", rotate_an_image(data_img_hopper));

