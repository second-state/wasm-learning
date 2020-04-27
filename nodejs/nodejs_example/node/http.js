const { fetch, download } = require('../pkg/nodejs_example.js');
  
fetch("https://raw.githubusercontent.com/second-state/nodejs-helper/master/LICENSE");
download("https://www.secondstate.io/", "test.html");
