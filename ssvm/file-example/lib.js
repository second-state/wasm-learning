let vm;
module.exports.file_demo = function() {
	return vm.Start();
};

const ssvm = require('ssvm');
const path = require('path').join(__dirname, 'target/wasm32-wasi/release/file-example.wasm');

vm = new ssvm.VM(path, {"EnableWasiStartFunction": true, env: process.env, args: process.argv, preopens:{'/': __dirname}});
