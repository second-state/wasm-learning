let vm;
module.exports.generate_key_pair = function() {
	return vm.Start();
};

const ssvm = require('ssvm');
const path = require('path').join(__dirname, 'target/wasm32-wasi/release/rust-rsa-example.wasm');

vm = new ssvm.VM(path, {"EnableWasiStartFunction": true, env: process.env, args: process.argv, preopens:{'/': __dirname}});
