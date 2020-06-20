const express = require('express');
const { get_svg } = require('../pkg/svg_lib.js');

const app = express();
const port = 8080;
app.use(express.static('public'));
app.use(express.urlencoded({ extended: false }));

app.get('/', (req, res) => res.redirect("/index.html"));

app.post('/draw', function (req, res) {
  var width = 800;
  var height = 400;
  var p = 50;
  var svg = get_svg(req.body.x, req.body.y, width, height, p, req.body.title)
  res.send(svg)
})

app.listen(port, () => console.log(`Listening at http://localhost:${port}`))

