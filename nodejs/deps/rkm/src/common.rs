use ndarray::{ArrayView1, ArrayView2, Ix, ScalarOperand};
use num::{Float, NumCast, Zero};
use std::cmp::PartialOrd;
use std::fmt::Debug;
use std::marker::Sync;
use std::ops::Add;

/// Numeric value trait, defines the types that can be used for the value of each dimension in a
/// data point.
pub trait Value:
    ScalarOperand + Add + Zero + Float + NumCast + PartialOrd + Copy + Debug + Sync + Send
{
}
impl<T> Value for T where
    T: ScalarOperand + Add + Zero + Float + NumCast + PartialOrd + Copy + Debug + Sync + Send
{
}

/// Find the square of the distance between two data points, given as Array rows.
pub fn distance_squared<V: Value>(point_a: &ArrayView1<V>, point_b: &ArrayView1<V>) -> V {
    let mut distance = V::zero();
    for i in 0..point_a.shape()[0] {
        let delta = point_a[i] - point_b[i];
        distance = distance + (delta * delta)
    }
    distance
}

/// Find the distance between two data points.
pub fn distance<V: Value>(point_a: &ArrayView1<V>, point_b: &ArrayView1<V>) -> V {
    let d_squared = distance_squared(point_a, point_b);
    d_squared.sqrt()
}

/// Find the closest mean to a particular data point.
pub fn closest_mean<V: Value>(point: &ArrayView1<V>, means: &ArrayView2<V>) -> Ix {
    assert!(means.dim().0 > 0);
    let mut iter = means.outer_iter().enumerate();
    if let Some(compare) = iter.next() {
        let mut index = compare.0;
        let mut shortest_distance = distance_squared(point, &compare.1);
        for compare in iter {
            let distance = distance_squared(point, &compare.1);
            if distance < shortest_distance {
                shortest_distance = distance;
                index = compare.0;
            }
        }
        return index;
    }
    0 // Should never hit this due to the assertion of the precondition
}
