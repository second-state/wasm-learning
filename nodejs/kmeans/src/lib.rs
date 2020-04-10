use wasm_bindgen::prelude::*;
use linfa_clustering::{KMeansHyperParams, KMeans, generate_blobs};
use ndarray::{Axis, array, Array2};
use ndarray_rand::rand::SeedableRng;
use rand_isaac::Isaac64Rng;

#[wasm_bindgen]
pub fn test_data () -> String {
    let points = array![[0., 1.], [-10., 20.], [-1., 10.]];
    return serde_json::to_string(&points).unwrap();
}

#[wasm_bindgen]
pub fn test_data_2 () -> String {
    let points = array![[-9., 20.5]];
    return serde_json::to_string(&points).unwrap();
}

#[wasm_bindgen]
pub fn generate_data (center_pts: &str, cluster_size: i32) -> String {
    let seed = 42;
    let mut rng = Isaac64Rng::seed_from_u64(seed);

    let expected_centroids: Array2<f64> = serde_json::from_str(center_pts).unwrap();

    let observations = generate_blobs(cluster_size as usize, &expected_centroids, &mut rng);
    return serde_json::to_string(&observations).unwrap();
}

#[wasm_bindgen]
pub fn fit (center_pts: &str, data_pts: &str) -> String {
    let seed = 42;
    let mut rng = Isaac64Rng::seed_from_u64(seed);

    let observations: Array2<f64> = serde_json::from_str(data_pts).unwrap();
    let expected_centroids: Array2<f64> = serde_json::from_str(center_pts).unwrap();

    let n_clusters = expected_centroids.len_of(Axis(0));
    let hyperparams = KMeansHyperParams::new(n_clusters)
        .tolerance(1e-2)
        .build();
    let model = KMeans::fit(hyperparams, &observations, &mut rng);
    return serde_json::to_string(&model).unwrap();
}

#[wasm_bindgen]
pub fn predict (p: &str, m: &str) -> String {
    let model: KMeans = serde_json::from_str(m).unwrap();
    let new_observation: Array2<f64> = serde_json::from_str(p).unwrap();

    let closest_cluster_index = model.predict(&new_observation);
    let closest_centroid = &model.centroids().index_axis(Axis(0), closest_cluster_index[0]);
    return serde_json::to_string(&closest_centroid).unwrap();
}
