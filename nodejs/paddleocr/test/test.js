
const {text } = require('../pkg/dcr_lib.js');
const {bounding_box } = require('../pkg/dcr_lib.js');


const fs = require('fs');

var img_src ="./test/img_12.jpg";
text(img_src);
bounding_box(img_src);

