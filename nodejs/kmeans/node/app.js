const { test_data, test_data_2, generate_data, fit, predict } = require('./kmeans_lib.js');

// var centers = JSON.parse(test_data());
// console.log(centers);
// var obs = JSON.parse(test_data_2());
// console.log(obs);

// A 3x2 array representing 3 points in a 2-D space
var centers = { v: 1, dim: [ 3, 2 ], data: [ 0, 1, -10, 20, -1, 10 ] };
var initial_guess_centers = { v: 1, dim: [ 3, 2 ], data: [ 0.1, 1.1, -10.1, 19.9, -1.1, 9.9 ] };
// A 1x2 array representing 1 point in a 2-D space
var obs = { v: 1, dim: [ 1, 2 ], data: [ -9, 20.5 ] }

// Generate 100 random points around each of the 3 center points
// This returns a 300x2 array
var sample = JSON.parse(generate_data(JSON.stringify(centers), 100));
console.log(sample);

// Fit a KMeans for the 3 clusters of sample data points
var model = JSON.parse(fit(JSON.stringify(initial_guess_centers), JSON.stringify(sample)));
console.log(model);

// Use the KMeans model to determine which cluster the new observed point belongs to
var result = JSON.parse(predict(JSON.stringify(obs), JSON.stringify(model)));
console.log(result);
