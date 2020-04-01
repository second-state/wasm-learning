let imports = {};
let vm;

/**
* @param {string} s
* @returns {string}
*/
module.exports.say = function(s) {
    return vm.RunString('say', s);
};

/**
* @param {string} s
* @returns {string}
*/
module.exports.obfusticate = function(s) {
    return vm.RunString('obfusticate', s);
};

/**
* @param {number} a
* @param {number} b
* @returns {number}
*/
module.exports.lowest_common_denominator = function(a, b) {
    return vm.RunInt('lowest_common_denominator', a, b);
};

/**
* @param {Uint8Array} v
* @returns {Uint8Array}
*/
module.exports.sha3_digest = function(v) {
    return vm.RunUint8Array('sha3_digest', v);
};

/**
* @param {Uint8Array} s
* @returns {Uint8Array}
*/
module.exports.keccak_digest = function(s) {
    return vm.RunUint8Array('keccak_digest', s);
};

const path = require('path').join(__dirname, 'functions_lib_bg.wasm');
const ssvm = require('ssvm');
vm = new ssvm.VM(path)

