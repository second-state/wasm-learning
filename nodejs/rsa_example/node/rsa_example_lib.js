let imports = {};
let vm;

/**
* @param {number} bits
* @returns {string}
*/
module.exports.generate_key_pair = function(bits) {
    return vm.RunString('generate_key_pair', bits);
};

/**
* @param {string} pk
* @param {Uint8Array} data
* @returns {Uint8Array}
*/
module.exports.decrypt = function(pk, data) {
    return vm.RunUint8Array('decrypt', pk, data);
};

/**
* @param {string} pk
* @param {Uint8Array} data
* @returns {Uint8Array}
*/
module.exports.encrypt = function(pk, data) {
    return vm.RunUint8Array('encrypt', pk, data);
};

const path = require('path').join(__dirname, 'rsa_example_lib_bg.wasm');
const ssvm = require('ssvm');
vm = new ssvm.VM(path)

