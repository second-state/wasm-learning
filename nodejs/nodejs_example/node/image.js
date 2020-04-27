const { resize_file } = require('../pkg/nodejs_example.js');
  
const dim = {
    width: 100,
    height: 100
};

resize_file(JSON.stringify([dim, 'cat.png', `test.png`]));
