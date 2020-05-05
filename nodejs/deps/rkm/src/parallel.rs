// Parallel implementation details
use crate::common::*;
use ndarray::{Array2, ArrayView2, Axis, Ix};
use rand::distributions::{Distribution, WeightedIndex};
use rand::prelude::*;
use rand::Rng;
use rayon::prelude::*;

/// Find the shortest distance between each data point and any of a set of mean points (parallel version).
pub fn closest_distance<V: Value>(means: &ArrayView2<V>, data: &ArrayView2<V>) -> Vec<V> {
    data.outer_iter()
        .into_par_iter()
        .map(|d| {
            let mut iter = means.outer_iter();
            let mut closest = distance_squared(&d, &iter.next().unwrap());
            for m in iter {
                let distance = distance_squared(&d, &m);
                if distance < closest {
                    closest = distance;
                }
            }
            closest
        })
        .collect()
}

/// This is a mean initialization method based on the [kmeans++](https://en.wikipedia.org/wiki/K-means%2B%2B)
/// initialization algorithm (parallel version).
pub fn initialize_plusplus<V: Value>(
    data: &ArrayView2<V>,
    k: usize,
    seed: Option<u64>,
) -> Array2<V> {
    assert!(k > 1);
    assert!(data.dim().0 > 0);
    let mut means = Array2::zeros((k as usize, data.shape()[1]));
    let mut rng = match seed {
        Some(seed) => SmallRng::seed_from_u64(seed),
        None => SmallRng::from_rng(rand::thread_rng()).unwrap(),
    };
    let data_len = data.shape()[0];
    let chosen = rng.gen_range(0, data_len) as isize;
    means
        .slice_mut(s![0..1, ..])
        .assign(&data.slice(s![chosen..(chosen + 1), ..]));
    for i in 1..k as isize {
        // Calculate the distance to the closest mean for each data point
        let distances = closest_distance(&means.slice(s![0..i, ..]).view(), &data.view());
        // Pick a random point weighted by the distance from existing means
        let distance_sum: f32 = distances
            .iter()
            .fold(0.0f32, |sum, d| sum + num::cast::<V, f32>(*d).unwrap());
        let weights: Vec<f32> = distances
            .par_iter()
            .map(|p| num::cast::<V, f32>(*p).unwrap() / distance_sum)
            .collect();
        let chooser = WeightedIndex::new(&weights).unwrap();
        let chosen = chooser.sample(&mut rng) as isize;
        means
            .slice_mut(s![i..(i + 1), ..])
            .assign(&data.slice(s![chosen..(chosen + 1), ..]));
    }
    means
}

/// Calculate the index of the mean each data point is closest to (euclidean distance) (parallel version).
pub fn calculate_clusters<V: Value>(data: &ArrayView2<V>, means: &ArrayView2<V>) -> Vec<Ix> {
    data.outer_iter()
        .into_par_iter()
        .map(|point| closest_mean(&point.view(), means))
        .collect()
}

/// Calculate means based on data points and their cluster assignments (parallel version)
pub fn calculate_means<V: Value>(
    data: &ArrayView2<V>,
    clusters: &Vec<Ix>,
    old_means: &ArrayView2<V>,
    k: usize,
) -> Array2<V> {
    // TODO: replace old_means parameter with just its dimension, or eliminate it completely; that's all we need
    let (mut means, counts) = clusters
        .par_iter()
        .zip(data.outer_iter().into_par_iter())
        .fold(
            || (Array2::zeros(old_means.dim()), vec![0; k]),
            |mut totals, point| {
                {
                    let mut sum = totals.0.index_axis_mut(Axis(0), *point.0);
                    let new_sum = &sum + &point.1;
                    sum.assign(&new_sum);
                    // TODO: file a bug about how + and += work with ndarray
                }
                totals.1[*point.0] += 1;
                totals
            },
        )
        .reduce(
            || (Array2::zeros(old_means.dim()), vec![0; k]),
            |new_means, subtotal| {
                let total = new_means.0 + subtotal.0;
                let count = new_means
                    .1
                    .iter()
                    .zip(subtotal.1.iter())
                    .map(|counts| counts.0 + counts.1)
                    .collect();
                (total, count)
            },
        );
    for i in 0..k {
        let mut sum = means.index_axis_mut(Axis(0), i);
        let new_mean = &sum / V::from(counts[i]).unwrap();
        sum.assign(&new_mean);
    }
    means
}
