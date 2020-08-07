const express = require('express');
const { solve } = require('../pkg/quadratic_lib.js');

const app = express();
const port = 8080;
app.use(express.static(__dirname + '/public'));
app.use(express.urlencoded({ extended: false }));
/*
var bodyParser = require('body-parser')
app.use(bodyParser.urlencoded({
  extended: true
})); 
*/

app.get('/', (req, res) => res.redirect("/index.html"));

app.post('/solve', function (req, res) {
  var a = parseFloat(req.body.a);
  var b = parseFloat(req.body.b);
  var c = parseFloat(req.body.c);
  res.send(solve([a, b, c]))
})

app.listen(port, () => console.log(`Listening at http://localhost:${port}`))

