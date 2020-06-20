const { get_svg } = require('../pkg/svg_lib.js');

var x = [1., 2., 3., 4., 5., 6., 7., 8., 9., 10., 11., 12.];
var y = [10., 3., 17., 20., 25., 23., 30., 38.2, 32.5, 40.8, 35.7, 21.7];
// console.log( JSON.stringify(x) );
var width = 800;
var height = 400;
var p = 50;
var title = "Demo";
var svg = get_svg(x, y, width, height, p, title)
console.log( svg );
