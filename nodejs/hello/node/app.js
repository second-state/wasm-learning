const { init, say } = require('./hello_lib.js');

(async () => {
  const wasm = await init();
  console.log(say(wasm, 'World!'));
})();
