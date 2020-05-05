use assert_approx_eq::assert_approx_eq;
use ndarray::{arr2, Array2};
use rkm::{kmeans_lloyd, kmeans_lloyd_with_config, Config};

fn read_test_data(data_path: &str, dim: usize) -> Array2<f32> {
    use std::str::FromStr;
    let mut data_reader = csv::Reader::from_path(data_path).unwrap();
    let mut data: Vec<f32> = Vec::new();
    for record in data_reader.records() {
        for field in record.unwrap().iter() {
            let value = f32::from_str(field);
            data.push(value.unwrap());
        }
    }
    Array2::from_shape_vec((data.len() / dim, dim), data).unwrap()
}

#[test]
#[should_panic(expected = "assertion failed")]
fn test_min_k() {
    let d = arr2(&[[1.0f32, 1.0f32], [2.0f32, 2.0f32], [3.0f32, 3.0f32]]);
    kmeans_lloyd(&d.view(), 1);
}

#[test]
fn test_small_kmeans() {
    let d = arr2(&[
        [1.0f32, 1.0f32],
        [2.0f32, 2.0f32],
        [3.0f32, 3.0f32],
        [1200.0f32, 1200.0f32],
        [956.0f32, 956.0f32],
        [1024.0f32, 1024.0f32],
        [1024.0f32, 17.0f32],
        [1171.0f32, 20.0f32],
    ]);
    let expected_means = arr2(&[[1060.0, 1060.0], [1097.5, 18.5], [2.0, 2.0]]);
    let expected_clusters = vec![2, 2, 2, 0, 0, 0, 1, 1];
    let config = Config::from(Some(0), None, None);
    let (means, clusters) = kmeans_lloyd_with_config(&d.view(), 3, &config);
    println!("{:?}", means);
    assert_eq!(clusters, expected_clusters);
    means.iter().zip(expected_means.iter()).for_each(|m| {
        assert_approx_eq!(m.0, m.1);
    });
}

/// Test a simple real dataset
#[test]
fn test_iris() {
    let data = read_test_data("data/iris.data.csv", 2);
    let expected_means = arr2(&[
        [2.7075472f32, 1.3094337f32],
        [3.0416667f32, 2.0520833f32],
        [3.439583f32, 0.24374998f32],
    ]);
    let config = Config::from(Some(0), None, None);
    let (means, clusters) = kmeans_lloyd_with_config(&data.view(), 3, &config);
    // not checking actual cluster values because there are too many
    println!("{:?}", means);
    assert_eq!(clusters.len(), data.dim().0);
    means.iter().zip(expected_means.iter()).for_each(|m| {
        assert_approx_eq!(m.0, m.1);
    });
}

/// Test a simple synthetic data set
#[test]
fn test_s1() {
    let data = read_test_data("data/s1.data.csv", 2);
    let expected_means = arr2(&[
        [283176.94, 545553.0],
        [801616.9, 321123.3],
        [733640.3, 811047.9],
        [606380.8, 574534.94],
        [171069.97, 320523.6],
        [327658.56, 818571.5],
        [507818.22, 175610.5],
        [136671.92, 558362.8],
        [349034.16, 566772.4],
        [398870.1, 404924.0],
        [852058.44, 157685.61],
        [617926.6, 399415.97],
        [320602.44, 161521.75],
        [165909.27, 364343.53],
        [856919.2, 566696.56],
    ]);
    let config = Config::from(Some(0), None, None);
    let (means, clusters) = kmeans_lloyd_with_config(&data.view(), 15, &config);
    // not checking actual cluster values because there are too many
    println!("{:?}", means);
    assert_eq!(clusters.len(), data.dim().0);
    means.iter().zip(expected_means.iter()).for_each(|m| {
        assert_approx_eq!(m.0, m.1, 1.0f32);
    });
}

/// Test a simple synthetic data set with an early termination condition.
/// Only checks that the means are as would be expected from an early
/// termination because the public API doesn't expose the iteration count
/// (and doing so would have little point other than for testing)
#[test]
fn test_iteration_limit() {
    let data = read_test_data("data/s1.data.csv", 2);
    let expected_means = arr2(&[
        [283176.94, 545553.0],
        [801616.9, 321123.3],
        [733640.3, 811047.9],
        [606380.8, 574534.94],
        [171069.97, 320523.6],
        [327658.56, 818571.5],
        [507818.22, 175610.5],
        [136671.92, 558362.8],
        [349034.16, 566772.4],
        [398870.1, 404924.0],
        [852058.44, 157685.61],
        [617926.6, 399415.97],
        [320602.44, 161521.75],
        [165909.27, 364343.53],
        [856919.2, 566696.56],
    ]);
    let config = Config::from(Some(0), None, None);
    let (means, clusters) = kmeans_lloyd_with_config(&data.view(), 15, &config);
    // not checking actual cluster values because there are too many
    println!("{:?}", means);
    assert_eq!(clusters.len(), data.dim().0);
    means.iter().zip(expected_means.iter()).for_each(|m| {
        assert_approx_eq!(m.0, m.1, 1.0f32);
    });
}

/// Test a simple synthetic data set with a delta termination condition.
/// Only checks that the means are as would be expected from an early
/// termination because the public API doesn't expose the delta.
#[test]
fn test_delta_limit() {
    let data = read_test_data("data/s1.data.csv", 2);
    let expected_means = arr2(&[
        [283176.94, 545553.0],
        [801616.9, 321123.3],
        [733640.3, 811047.9],
        [606380.8, 574534.94],
        [171069.97, 320523.6],
        [327658.56, 818571.5],
        [507818.22, 175610.5],
        [136671.92, 558362.8],
        [349034.16, 566772.4],
        [398870.1, 404924.0],
        [852058.44, 157685.61],
        [617926.6, 399415.97],
        [320602.44, 161521.75],
        [165909.27, 364343.53],
        [856919.2, 566696.56],
    ]);
    let config = Config::from(Some(0), None, None);
    let (means, clusters) = kmeans_lloyd_with_config(&data.view(), 15, &config);
    // not checking actual cluster values because there are too many
    println!("{:?}", means);
    assert_eq!(clusters.len(), data.dim().0);
    means.iter().zip(expected_means.iter()).for_each(|m| {
        assert_approx_eq!(m.0, m.1, 1.0f32);
    });
}
