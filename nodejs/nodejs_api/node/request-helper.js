const req = require('sync-request');

function request(method, url) {
  var res = req(method, url);
  return JSON.stringify(res);
}

function fetch(url) {
  var res = req('GET', url);
  return res.getBody();
}

function fetch_as_string(url) {
  var res = req('GET', url);
  return res.getBody('utf8');
}

function request_with_options(method, url, options) {
  var res = req(method, url, JSON.parse(options));
  return JSON.stringify(res);
}

module.exports = { request, fetch, fetch_as_string, request_with_options }
