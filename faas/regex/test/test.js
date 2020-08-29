const { match_text } = require('../pkg/match_regex_lib.js');

var p = ["\\d{4}-\\d{2}-\\d{2}", "On 2009-01-03, Satoshi Nakamoto launched the Bitcoin blockchain. The price reached a high of $19,783.06 on 2017-12-17 and dropped to a low of $3,300 on 2018-12-07."];
console.log( JSON.stringify(p) );
console.log( match_text(JSON.stringify(p)) );
