const { generate_key_pair } = require('./lib.js');

console.log( "Generate Key Pair:" );
if (generate_key_pair(2048) != 0) {
	console.log( "Fail!" );
} else {
	console.log( "Success!" );
}
