const { init, say } = require('./hello_lib.js');

(async () => {
  await init();
  console.log(say('World!'));
})();
