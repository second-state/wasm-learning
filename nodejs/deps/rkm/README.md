# RKM - Rust k-means #

[![Build Status](https://travis-ci.org/genbattle/rkm.svg?branch=master)](https://travis-ci.org/genbattle/rkm)[![docs](https://docs.rs/rkm/badge.svg)](https://docs.rs/rkm/latest/rkm/) [![crates.io](https://img.shields.io/crates/v/rkm.svg)](https://crates.io/crates/rkm)

A simple [Rust](https://www.rust-lang.org) implementation of the [k-means clustering algorithm](http://en.wikipedia.org/wiki/K-means_clustering) based on a C++ implementation, [dkm](https://github.com/genbattle/dkm).

This implementation is generic, and will accept any type that satisfies the `Value` trait requirements. At a minimum, numeric floating point types built into Rust (`f32` and `f64`) should be supported. Rayon is used for parallelism to improve scalability at the cost of some performance on small data sets.

The `parallel` feature enables parallelism to speed up the algorithm in complex cases. The parallel algorithm may be slower than the non-parallel algorithm for small data sets, but is much faster for data sets with high dimensionality. Make sure you benchmark your use case with both configurations before deciding which to use.

Known to compile against Rust stable 1.41.0.

### Usage ###

Calculate the k-means clusters for a set of data by calling `rkm::kmeans_lloyd` with your dataset in a 2D `ndarray` array and the number of clusters you would like to segment the data into. The return value will be a tuple containing the cluster means/centroids (as a 2D `ndarray`) and a `Vec` of indices that map each of the input data points to an element of the means array.

e.g.

```rust
// load an `ndarray.Array2` containing the points
let data = read_test_data();
// split the data into 3 clusters, iterating until the algorithm converges
let (means, clusters) = rkm::kmeans_lloyd(&data.view(), 3);
println!(
    "data:\n{:?}\nmeans:\n{:?}\nclusters:\n{:?}",
    data, means, clusters
);
```

Another variation of the algorithm can be used via the `rkm::kmeans_lloyd_with_config` function which takes a reference to an additional `Config` struct as a reference. This config struct can be used to change the termination behavior of the algorithm, or change the random seed used for initialization.

```rust
let config = Config::from(Some((7 as u128).to_le_bytes()), None, None);
let (means, clusters) = kmeans_lloyd_with_config(&data.view(), 3, &config);
```

You can change the termination behavior of the algorithm by specifying a maximum number of iterations to terminate at.

```rust
let config = Config::from(None, Some(10), None);
let (means, clusters) = kmeans_lloyd_with_config(&data.view(), 3, &config);
```

The other termination condition that can be set is the minimum delta in mean positions observed between iterations; if none of the means change by more than this amount between successive iterations, the algorithm terminates.

```rust
let config = Config::from(None, None, Some(12.7f));
let (means, clusters) = kmeans_lloyd_with_config(&data.view(), 3, &config);
```

You may also specify both limits and first one to be triggered will stop the algorithm. The default termination criteria for `rkm::kmeans_lloyd` or when an empty `Config` is used with `rkm:::kmeans_lloyd_with_config` is to terminate when the means don't change at all between iterations. It may be prudent to set an arbitrarily high iteration limit to prevent infinite oscillations between a small number of states in some non-convergent edge cases.

See `examples/example.rs` for a simple usage example.

### Data ###

 A small set of benchmarks for this library is included in `benches/bench.rs`. The data sets are as follows:

`iris.data.csv` natural data taken from measurements of different iris plants. 150 points, 2 dimensions, 3 clusters. Source: [UCI machine learning repository](https://archive.ics.uci.edu/ml/datasets/Iris).

`s1.data.csv` synthetic data. 5000 points, 2 dimensions, 15 clusters. Source: P. Fränti and O. Virmajoki, "Iterative shrinking method for clustering problems", _Pattern Recognition_, 39 (5), 761-765, May 2006.

`birch3.data.csv` synthetic data large set. 100000 points, 2 dimensions, 100 clusters. Source: Zhang et al., "BIRCH: A new data clustering algorithm and its applications", _Data Mining and Knowledge Discovery_, 1 (2), 141-182, 1997

`dim128.data.csv` synthetic data with high dimensionality. 1024 points, 128 dimensions, 16 clusters. Source: P. Fränti, O. Virmajoki and V. Hautamäki, "Fast agglomerative clustering using a k-nearest neighbor graph", _IEEE Trans. on Pattern Analysis and Machine Intelligence_, 28 (11), 1875-1881, November 2006

Compared to [dkm](https://github.com/genbattle/dkm) this implementation is slower for the small iris and s1 data sets, but faster for the `dim128` and `birch3` data sets.

### Licensing ###

 This code is licensed under the MIT license.
