const { circumference, area, solve, draw } = require('./json_io_lib.js');

var x = 10.;
console.log( circumference(JSON.stringify(x)) );

var x = [10., 5.];
console.log( area(JSON.stringify(x)) );

var x = [2., 5., -3.];
console.log( solve(JSON.stringify(x)) );

var x = [{x:1.5, y:3.8}, {x:2.5, y:5.8}, "A thin red line"];
console.log( draw(JSON.stringify(x)) );
