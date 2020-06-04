let vm;
/**
* @param {number} bits
* @returns {string}
*/
module.exports.generate_key_pair = function(bits) {
	return vm.Run("_start", "generate_key_pair", bits);
};

const ssvm = require('ssvm');
const path = require('path').join(__dirname, 'target/wasm32-wasi/release/rust-rsa-example.wasm');

vm = new ssvm.VM(path, {});
