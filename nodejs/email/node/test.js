const { send_email } = require('../pkg/send_email_lib.js');

var s = {"from":"michael@secondstate.io", "token":"SG.xxx", "to":"juntao_yuan@yahoo.com", "subject":"This is a HTTP Proxy test", "mime":"text/plain", "body":"It worked!"};
console.log( send_email(s) );
