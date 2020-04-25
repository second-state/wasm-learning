const fs = require('fs');
const { resize_file } = require('../pkg/nodejs_api.js');
  
(async () => {
  const dim = {
    width: 100,
    height: 100
  };

  resize_file(JSON.stringify([dim, './cat.png', `./cat-${dim.width}-${dim.height}.png`]));
})();
