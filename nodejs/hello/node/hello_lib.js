let imports = {};
let vm;

/**
* @param {string} s
* @returns {string}
*/
module.exports.say = function(s) {
    return vm.RunString('say', s);
};

const path = require('path').join(__dirname, 'hello_lib_bg.wasm');
const ssvm = require('ssvm');
vm = new ssvm.VM(path)

