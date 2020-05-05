#![warn(clippy::all)]
/// This crate contains a simple implementation of the
/// [k-means clustering algorithm](http://en.wikipedia.org/wiki/K-means_clustering).
#[macro_use(s)]
extern crate ndarray;

use ndarray::{Array2, ArrayView2};

cfg_if::cfg_if! {
    if #[cfg(feature = "parallel")] {
        mod parallel;
        use self::parallel::*;
    } else {
        mod serial;
        use self::serial::*;
    }
}
mod common;
pub use common::Value;
use common::*;

/// Algorithm configuration parameters.
///
/// Passed to `rkm::kmeans_lloyd` to specify:
///
/// * Random number generator seed.
/// * The maximum number of iterations to terminate the algorithm at.
/// * The minimum delta for all means from iteration to iteration.
///
/// The algorithm will terminate if the maximum number of iterations is exceeded, or none of the means
/// change by at least the minimum delta distance, or the algorithm converges.
#[derive(Debug)]
pub struct Config<V: Value> {
    random_seed: Option<u64>,
    max_iterations: Option<u64>,
    min_delta: Option<V>,
}

impl<V: Value> Config<V> {
    /// Create a new config struct from a partial or complete set of parameters
    pub fn from(
        random_seed: Option<u64>,
        max_iterations: Option<u64>,
        min_delta: Option<V>,
    ) -> Config<V> {
        Config {
            random_seed,
            max_iterations,
            min_delta,
        }
    }

    /// Create an empty config struct
    pub fn empty() -> Config<V> {
        Config {
            random_seed: None,
            max_iterations: None,
            min_delta: None,
        }
    }
}

/// Calculate magnitude deltas between two sets of points
fn deltas<V: Value>(old_means: &ArrayView2<V>, new_means: &ArrayView2<V>) -> Vec<V> {
    old_means
        .outer_iter()
        .zip(new_means.outer_iter())
        .map(|points| distance(&points.0, &points.1))
        .collect()
}

/// Check if any of the deltas between two sets of points have exceeded a given limit
fn deltas_below_limit<V: Value>(
    old_means: &ArrayView2<V>,
    new_means: &ArrayView2<V>,
    limit: V,
) -> bool {
    let deltas = deltas(old_means, new_means);
    deltas.into_iter().all(|d| d < limit)
}

/// Calculate means and cluster assignments for the given data and number of clusters (k) with
/// a set of optional parameters.
///
/// Returns a tuple containing the means (as a 2D ndarray) and a `Vec` of indices that
/// map into the means ndarray and correspond elementwise to each input data point to give
/// the cluster assignments for each data point. Takes a `Config` object which can be used to
/// optionally specify:
///
/// * Random number seed (for initialization)
/// * Maximum number of iterations
/// * Minimum mean delta distance
///
/// The algorithm will terminate when convergence is reached, or the number of iterations
/// equals the maximum, or none of the means change by at least the minimum delta distance.
pub fn kmeans_lloyd_with_config<V: Value>(
    data: &ArrayView2<V>,
    k: usize,
    config: &Config<V>,
) -> (Array2<V>, Vec<usize>) {
    assert!(k > 1);
    assert!(data.dim().0 >= k);

    let mut old_means = initialize_plusplus(data, k, config.random_seed);
    let mut clusters = calculate_clusters(data, &old_means.view());
    let mut means = calculate_means(data, &clusters, &old_means.view(), k);
    let mut iteration_count = 0;

    while means != old_means
        && !(config.max_iterations.is_some() && iteration_count == config.max_iterations.unwrap())
        && !(config.min_delta.is_some()
            && deltas_below_limit(&old_means.view(), &means.view(), config.min_delta.unwrap()))
    {
        clusters = calculate_clusters(data, &means.view());
        old_means = means;
        means = calculate_means(data, &clusters, &old_means.view(), k);
        iteration_count += 1;
    }

    (means, clusters)
}

/// Calculate means and cluster assignments for the given data and number of clusters (k).
///
/// Returns a tuple containing the means (as a 2D ndarray) and a `Vec` of indices that
/// map into the means ndarray and correspond elementwise to each input data point to give
/// the cluster assignments for each data point.
pub fn kmeans_lloyd<V: Value>(data: &ArrayView2<V>, k: usize) -> (Array2<V>, Vec<usize>) {
    kmeans_lloyd_with_config(data, k, &Config::empty())
}

#[cfg(test)]
mod tests {
    use assert_approx_eq::assert_approx_eq;

    #[test]
    fn test_distance() {
        use super::distance_squared;
        use ndarray::arr1;
        let a = arr1(&[1.0f32, 1.0f32]);
        let b = arr1(&[2.0f32, 2.0f32]);
        let c = arr1(&[1200.0f32, 1200.0f32]);
        let d = arr1(&[1.0f32, 1.0f32]);
        let e = arr1(&[1200.0f32, 1200.0f32]);
        assert_approx_eq!(distance_squared(&a.view(), &b.view()), 2.0f32);
        assert_approx_eq!(distance_squared(&a.view(), &c.view()), 2_875_202.0f32);
        assert_approx_eq!(distance_squared(&d.view(), &e.view()), 2_875_202.0f32);
    }

    #[test]
    fn test_closest_distances() {
        use super::closest_distance;
        use ndarray::arr2;
        let a = arr2(&[
            [1.0f32, 1.0f32],
            [2.0f32, 2.0f32],
            [100.0f32, 4.0f32],
            [3.0f32, 100.0f32],
            [7.0f32, 88.0f32],
            [70.0f32, 20.0f32],
            [22.0f32, 12.0f32],
        ]);

        let m = arr2(&[[0.0f32, 0.0f32], [100.0f32, 0.0f32], [0.0f32, 100.0f32]]);
        assert_eq!(
            closest_distance(&m.view(), &a.view()),
            vec![2.0f32, 8.0f32, 16.0f32, 9.0f32, 193.0f32, 1300.0f32, 628.0f32]
        );
    }

    #[test]
    fn test_closest_mean() {
        use super::closest_mean;
        use ndarray::{arr1, arr2};
        {
            let p = arr1(&[2.0f32, -1.0f32]);
            let m = arr2(&[
                [1.0f32, 1.0f32],
                [5.0f32, 100.0f32],
                [44.0f32, 65.0f32],
                [-5.0f32, -6.0f32],
            ]);
            assert_eq!(closest_mean(&p.view(), &m.view()), 0);
        }

        {
            let p = arr1(&[1024.0f32, 768.0f32]);
            let m = arr2(&[
                [1.0f32, 1.0f32],
                [5.0f32, 100.0f32],
                [512.0f32, 768.0f32],
                [-5.0f32, -6.0f32],
            ]);
            assert_eq!(closest_mean(&p.view(), &m.view()), 2);
        }
    }

    #[test]
    fn test_calculate_means() {
        use super::calculate_means;
        use ndarray::arr2;
        {
            let d = arr2(&[
                [0.0f32, 0.0f32],
                [2.0f32, 2.0f32],
                [4.0f32, 5.0f32],
                [5.0f32, 100.0f32],
                [128.0f32, 300.0f32],
                [512.0f32, 768.0f32],
                [-5.0f32, -6.0f32],
                [5.0f32, 6.0f32],
            ]);
            let c = vec![0, 0, 1, 1, 2, 2, 3, 3];
            let m = arr2(&[
                [0.0f32, 0.0f32],
                [0.0f32, 0.0f32],
                [0.0f32, 0.0f32],
                [0.0f32, 0.0f32],
            ]);
            let expected_means = arr2(&[
                [1.0f32, 1.0f32],
                [4.50f32, 52.5f32],
                [320.0f32, 534.0f32],
                [0.0f32, 0.0f32],
            ]);
            assert_eq!(calculate_means(&d.view(), &c, &m.view(), 4), expected_means);
            calculate_means(&d.view(), &c, &m.view(), 4)
                .iter()
                .zip(expected_means.iter())
                .for_each(|m| {
                    assert_approx_eq!(m.0, m.1);
                });
        }
    }
}
